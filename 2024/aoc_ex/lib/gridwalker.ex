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

  @star_backward [
    {1, -1},
    {-1, 1}
  ]
  @star_forward [
    {-1, -1},
    {1, 1}
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

  defp starts_with_word?(grid, [char | rest], {row, col}, {dx, dy}) do
    case get_char(grid, row, col) do
      ^char -> starts_with_word?(grid, rest, {row + dx, col + dy}, {dx, dy})
      _ -> false
    end
  end

  defp starts_with_word?(_grid, [], _position, _direction), do: true

  ########
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
        forward_match =
          match_direction?(grid, {mid_row, mid_col}, @star_forward, [
            first_char,
            middle_char,
            last_char
          ])

        backward_match =
          match_direction?(grid, {mid_row, mid_col}, @star_backward, [
            first_char,
            middle_char,
            last_char
          ])

        forward_match_rev =
          match_direction?(grid, {mid_row, mid_col}, @star_forward, [
            last_char,
            middle_char,
            first_char
          ])

        backward_match_rev =
          match_direction?(grid, {mid_row, mid_col}, @star_backward, [
            last_char,
            middle_char,
            first_char
          ])

        IO.puts("Checking position {#{mid_col}, #{mid_row}}:")
        IO.puts("  Forward match: #{forward_match}")
        IO.puts("  Backward match: #{backward_match}")
        IO.puts("  Forward match rev: #{forward_match_rev}")
        IO.puts("  Backward match rev: #{backward_match_rev}")
        ok = (forward_match or forward_match_rev) and (backward_match or backward_match_rev)
        IO.puts("  OK: #{ok}")
        IO.puts("++++++")
        ok

      _ ->
        false
    end
  end

  defp match_direction?(grid, {mid_row, mid_col}, directions, [
         first_char,
         _middle_char,
         last_char
       ]) do
    Enum.zip(directions, [first_char, last_char])
    |> Enum.all?(fn {{row_delta, col_delta}, expected_char} ->
      get_char(grid, mid_row + row_delta, mid_col + col_delta) == expected_char
    end)
  end

  #####

  defp get_char(_grid, row, col) when row < 0 or col < 0, do: nil

  defp get_char(grid, row, col) do
    case Enum.at(grid, row) do
      nil -> nil
      row -> Enum.at(row, col)
    end
  end
end
