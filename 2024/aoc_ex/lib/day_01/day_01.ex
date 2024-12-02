defmodule ElixirAoc.Day01 do
  def run do
    IO.puts("Running Aoc.Day01...")
    part1()
  end

  def part1 do
    {:ok, content} = ElixirAoc.FileReader.read_file("/day01/input1.txt")
    IO.puts(content)
  end
end
