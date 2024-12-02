defmodule ElixirAoc.Day01 do
  def run(part \\ "part1") do
    IO.puts("Running Aoc.Day01 for #{part}...")
    {:ok, content} = ElixirAoc.FileReader.read_file("/day_01/input.txt")

    case part do
      "part1" -> part1(content) |> IO.puts()
      "part2" -> part2(content) |> IO.puts()
      _ -> IO.puts("Invalid part: #{part}")
    end
  end

  def part1(content) do
    {left, right} =
      content
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        # Convert each list to a tuple
        String.split(line, ~r/\s+/, trim: true)
        |> Enum.map(&String.to_integer/1)
        |> List.to_tuple()
      end)
      # Unzip into two lists
      |> Enum.unzip()

    left = Enum.sort(left)
    right = Enum.sort(right)

    Enum.zip(left, right)
    |> Enum.map(fn {l, r} -> abs(l - r) end)
    |> Enum.sum()
  end

  def part2(content) do
    content
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      String.split(line, ~r/\s+/, trim: true)
      |> Enum.map(&String.to_integer/1)
      |> List.to_tuple()
    end)
    |> Enum.unzip()
    |> (fn {left, right} ->
          left
          |> Enum.map(fn l ->
            Enum.filter(right, fn r -> r == l end)
            |> Enum.count()
            |> (&(&1 * l)).()
          end)
        end).()
    |> Enum.sum()
  end
end
