from core.greeting import greet


def handler() -> str:
    return greet("api")
