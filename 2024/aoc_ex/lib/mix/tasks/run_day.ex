defmodule Mix.Tasks.RunDay do
  use Mix.Task

  @shortdoc "Run a specific AoC day"
  def run([day]) do
    day
    |> String.to_integer()
    |> ElixirAoc.run()
  end

  def run(_) do
    IO.puts("Usage: mix run_day <day>")
  end
end
