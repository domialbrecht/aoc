defmodule ElixirAoc.Day04 do
  alias ElixirAoc.Grid2d.Search, as: Grid2d

  def run(part \\ "part1") do
    IO.puts("Running Aoc.Day04 for #{part}...")
    {:ok, content} = ElixirAoc.FileReader.read_file("/day_04/input.txt")

    case part do
      "part1" -> part1(content) |> IO.puts()
      "part2" -> part2(content) |> IO.puts()
      _ -> IO.puts("Invalid part: #{part}")
    end
  end

  def part1(content) do
    content
    |> char_grid()
    |> Grid2d.count_word("XMAS")
  end

  def char_grid(content) do
    content
    |> String.split(~r/\n/, trim: true)
    |> Enum.map(fn line ->
      line
      |> String.split("", trim: true)
    end)
  end

  def part2(content) do
    content
    |> char_grid()
    |> Grid2d.count_stars("A", "MAS")
  end
end
