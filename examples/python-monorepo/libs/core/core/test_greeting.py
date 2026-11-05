from core.greeting import greet


def test_greet() -> None:
    assert greet("api") == "Hello from api"
