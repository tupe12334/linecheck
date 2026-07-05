// Command go-example calls linecheck from Go via wazero, a pure-Go WASI runtime
// (no cgo, no system wasmtime install) loading the wasm32-wasip1 build of
// crates/wasm-wasi. Build that module first:
//
//	cargo build -p linecheck-wasm-wasi --target wasm32-wasip1 --release
//	go run . [path/to/linecheck_wasm_wasi.wasm]
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

const defaultWasmPath = "../../target/wasm32-wasip1/release/linecheck_wasm_wasi.wasm"

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}

func run() error {
	path := defaultWasmPath
	if len(os.Args) > 1 {
		path = os.Args[1]
	}
	wasmBytes, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("read %s (build it with cargo first): %w", path, err)
	}

	ctx := context.Background()
	runtime := wazero.NewRuntime(ctx)
	defer runtime.Close(ctx)

	if _, err := wasi_snapshot_preview1.Instantiate(ctx, runtime); err != nil {
		return err
	}
	mod, err := runtime.Instantiate(ctx, wasmBytes)
	if err != nil {
		return err
	}

	result, err := checkContent(ctx, mod, "src/main.rs", "one\ntwo\nthree\n", "")
	if err != nil {
		return err
	}
	if err := assertField(result, "status", "ok"); err != nil {
		return err
	}
	if err := assertField(result, "lines", float64(3)); err != nil {
		return err
	}
	fmt.Println("ok:", result)

	errResult, err := checkContent(ctx, mod, "src/main.rs", "one\n", "not: [valid")
	if err != nil {
		return err
	}
	if err := assertField(errResult, "status", "error"); err != nil {
		return err
	}
	fmt.Println("error:", errResult)
	return nil
}

func assertField(jsonResult, key string, want any) error {
	var parsed map[string]any
	if err := json.Unmarshal([]byte(jsonResult), &parsed); err != nil {
		return fmt.Errorf("parse result JSON: %w", err)
	}
	if got := parsed[key]; got != want {
		return fmt.Errorf("expected %s=%v, got %v (full result: %s)", key, want, got, jsonResult)
	}
	return nil
}

// checkContent mirrors the linear-memory ABI documented in crates/wasm-wasi/src/lib.rs:
// write filename/content/config bytes via alloc, call check, read back the packed
// pointer/length JSON result, then free every buffer with dealloc.
func checkContent(ctx context.Context, mod api.Module, filename, content, configYAML string) (string, error) {
	alloc := mod.ExportedFunction("alloc")
	dealloc := mod.ExportedFunction("dealloc")
	check := mod.ExportedFunction("check")
	mem := mod.Memory()

	write := func(s string) (uint32, uint32) {
		b := []byte(s)
		if len(b) == 0 {
			return 0, 0
		}
		res, err := alloc.Call(ctx, uint64(len(b)))
		if err != nil {
			panic(err)
		}
		ptr := uint32(res[0])
		if !mem.Write(ptr, b) {
			panic("write out of range")
		}
		return ptr, uint32(len(b))
	}

	fPtr, fLen := write(filename)
	cPtr, cLen := write(content)
	gPtr, gLen := write(configYAML)

	packed, err := check.Call(ctx, uint64(fPtr), uint64(fLen), uint64(cPtr), uint64(cLen), uint64(gPtr), uint64(gLen))
	if err != nil {
		return "", err
	}
	outPtr := uint32(packed[0] >> 32)
	outLen := uint32(packed[0] & 0xFFFFFFFF)
	out, ok := mem.Read(outPtr, outLen)
	if !ok {
		return "", fmt.Errorf("result out of memory range")
	}
	result := string(out)

	for _, p := range [][2]uint32{{fPtr, fLen}, {cPtr, cLen}, {gPtr, gLen}, {outPtr, outLen}} {
		if p[1] == 0 {
			continue
		}
		if _, err := dealloc.Call(ctx, uint64(p[0]), uint64(p[1])); err != nil {
			return "", err
		}
	}

	return result, nil
}
