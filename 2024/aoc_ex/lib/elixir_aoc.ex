defmodule ElixirAoc do
  def run(day, part \\ "part1") do
    module = resolve_day_module(day)

    if Code.ensure_loaded?(module) do
      apply(module, :run, [part])
    else
      IO.puts("Day #{day} is not implemented.")
    end
  end

  defp resolve_day_module(day) do
    Module.concat(
      __MODULE__,
      String.to_atom("Day#{String.pad_leading(Integer.to_string(day), 2, "0")}")
    )
  end
end

