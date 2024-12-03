defmodule ElixirAoc.Day02 do
  def run(part \\ "part1") do
    IO.puts("Running Aoc.Day02 for #{part}...")
    {:ok, content} = ElixirAoc.FileReader.read_file("/day_02/input.txt")

    case part do
      "part1" -> part1(content) |> IO.puts()
      "part2" -> part2(content) |> IO.puts()
      _ -> IO.puts("Invalid part: #{part}")
    end
  end

  def part1(content) do
    reports =
      content
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        String.split(line, ~r/\s+/, trim: true)
        |> Enum.map(fn word ->
          String.to_integer(word)
        end)
      end)

    reports
    |> Enum.filter(fn report ->
      is_safe_report(report)
    end)
    |> Enum.count()
  end

  def part2(content) do
    reports =
      content
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        String.split(line, ~r/\s+/, trim: true)
        |> Enum.map(fn word ->
          String.to_integer(word)
        end)
      end)

    reports
    |> Enum.filter(fn report ->
      is_safe_report_damp(report)
    end)
    |> Enum.count()
  end

  defp is_safe_report(list) do
    Enum.reduce_while(Enum.zip(list, tl(list)), :unknown, fn
      {a, b}, state ->
        cond do
          abs(a - b) > 3 -> {:halt, :unsafe}
          state in [:unknown, :increasing] and a < b -> {:cont, :increasing}
          state in [:unknown, :decreasing] and a > b -> {:cont, :decreasing}
          true -> {:halt, :unsafe}
        end
    end) != :unsafe
  end

  defp is_safe_report_damp(list) do
    if is_safe_report(list) do
      true
    else
      list
      |> Enum.with_index()
      |> Enum.any?(fn {_level, index} ->
        sublist = List.delete_at(list, index)
        is_safe_report(sublist)
      end)
    end
  end
end
