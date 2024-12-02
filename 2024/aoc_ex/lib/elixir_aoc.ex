defmodule ElixirAoc do
  def hello do
    :aoc
  end

  def run(day) do
    module =
      Module.concat(
        ElixirAoc,
        String.to_atom("Day#{String.pad_leading(Integer.to_string(day), 2, "0")}")
      )

    if Code.ensure_loaded?(module) do
      apply(module, :run, [])
    else
      IO.puts("Day #{day} is not implemented.")
    end
  end
end
