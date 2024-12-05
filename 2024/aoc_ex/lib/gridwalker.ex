defmodule ElixirAoc.Grid2d.Search do
  @directions [
    {0, 1},
    {0, -1},
    {1, 0},
    {-1, 0},
    {1, 1},
    {-1, -1},
    {1, -1},
    {-1, 1}
  ]

  @star_directions [
    {1, 1},
    {-1, -1},
    {1, -1},
    {-1, 1}
  ]

  def count_word(grid, word) do
    word_chars = String.graphemes(word)

    grid
    |> Enum.with_index()
    |> Enum.flat_map(fn {row, row_idx} ->
      Enum.with_index(row)
      |> Enum.flat_map(fn {_, col_idx} ->
        Enum.map(@directions, fn direction ->
          if starts_with_word?(grid, word_chars, {row_idx, col_idx}, direction), do: 1, else: 0
        end)
      end)
    end)
    |> Enum.sum()
  end

  def count_stars(grid, middle_char, word) do
    word_chars = String.graphemes(word)

    grid
    |> Enum.with_index()
    |> Enum.flat_map(fn {row, row_idx} ->
      Enum.with_index(row)
      |> Enum.map(fn {_, col_idx} ->
        if star_matches?(grid, {row_idx, col_idx}, middle_char, word_chars) do
          1
        else
          0
        end
      end)
    end)
    |> Enum.sum()
  end

  defp star_matches?(grid, {mid_row, mid_col}, middle_char, [first_char, _mid_char, last_char]) do
    case get_char(grid, mid_row, mid_col) do
      ^middle_char ->
        Enum.all?(@star_directions, fn {dx, dy} ->
          case get_char(grid, mid_row + dx, mid_col + dy) do
            ^first_char -> true
            ^last_char -> true
            _ -> false
          end
        end)

      _ ->
        false
    end
  end

  defp starts_with_word?(grid, [char | rest], {row, col}, {dx, dy}) do
    case get_char(grid, row, col) do
      ^char -> starts_with_word?(grid, rest, {row + dx, col + dy}, {dx, dy})
      _ -> false
    end
  end

  defp starts_with_word?(_grid, [], _position, _direction), do: true

  defp get_char(_grid, row, col) when row < 0 or col < 0, do: nil

  defp get_char(grid, row, col) do
    case Enum.at(grid, row) do
      nil -> nil
      row -> Enum.at(row, col)
    end
  end
end
