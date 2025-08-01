"""Command-line interface."""

import typer


app: typer.Typer = typer.Typer()


@app.command(name="robust-maturin-demo")
def main() -> None:
    """Robust Maturin Demo."""


if __name__ == "__main__":
    app()  # pragma: no cover
