defmodule ElixirAoc.Day00 do
  def run(part \\ "part1") do
    IO.puts("Running Aoc.Day00 for #{part}...")
    {:ok, content} = ElixirAoc.FileReader.read_file("/day_00/input.txt")

    case part do
      "part1" -> part1(content) |> IO.puts()
      "part2" -> part2(content) |> IO.puts()
      _ -> IO.puts("Invalid part: #{part}")
    end
  end

  def part1(_content) do
    :todo
  end

  def part2(_content) do
    :todo
  end
end
