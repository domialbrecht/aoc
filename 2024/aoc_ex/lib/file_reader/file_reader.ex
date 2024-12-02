defmodule ElixirAoc.FileReader do
  def read_file(filename) do
    priv_dir = :code.priv_dir(:elixir_aoc)
    file_path = Path.join([priv_dir, filename])

    case File.read(file_path) do
      {:ok, content} ->
        {:ok, content}

      {:error, reason} ->
        {:error, reason}
    end
  end
end
