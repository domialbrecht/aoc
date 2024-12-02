defmodule Mix.Tasks.RunDay do
  use Mix.Task

  @shortdoc "Run a specific AoC day and part"
  def run([day, part]) do
    ElixirAoc.run(String.to_integer(day), part)
  end

  def run(_) do
    IO.puts("Usage: mix run_day <day> [part1|part2]")
    IO.puts("Example: mix run_day 01 part1")
  end
end

