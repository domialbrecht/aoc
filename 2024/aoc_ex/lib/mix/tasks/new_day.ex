defmodule Mix.Tasks.NewDay do
  use Mix.Task

  @shortdoc "Creates a new AoC day module by copying an existing template"
  @moduledoc """
  Creates a new Advent of Code day module by copying the `day_template` folder
  and replacing `Day00` with the specified day.

  ## Examples
      mix new_day 1
      mix new_day 25
  """

  def run([day]) do
    day_number = String.pad_leading(day, 2, "0")
    source_dir = "lib/day_template"
    target_dir = "lib/day_#{day_number}"

    if File.exists?(target_dir) do
      Mix.raise("Day #{day_number} already exists at #{target_dir}")
    else
      copy_and_replace(
        source_dir,
        target_dir,
        day_number
      )

      Mix.shell().info("Day #{day_number} created: #{target_dir}")
    end

    source_dir = "test/day_template"
    target_dir = "test/day_#{day_number}"

    if File.exists?(target_dir) do
      Mix.raise("Day Test #{day_number} already exists at #{target_dir}")
    else
      copy_and_replace(
        source_dir,
        target_dir,
        day_number
      )

      Mix.shell().info("Day Test #{day_number} created: #{target_dir}")
    end
  end

  def run(_) do
    Mix.shell().info("""
    Usage:
      mix new_day <day>

    Example:
      mix new_day 1
    """)
  end

  defp copy_and_replace(source_dir, target_dir, day_number) do
    # Copy the source directory recursively
    File.cp_r!(source_dir, target_dir)

    # Replace placeholders in filenames and file contents
    rename_files(target_dir, "day_00", "day_#{day_number}")
    replace_in_files(target_dir, "Day00", "Day#{day_number}")
    replace_in_files(target_dir, "day_00", "day_#{day_number}")
  end

  defp rename_files(dir, file_placeholder, file_replacement) do
    dir
    |> File.ls!()
    |> Enum.each(fn file ->
      old_path = Path.join(dir, file)

      if File.dir?(old_path) do
        rename_files(old_path, file_placeholder, file_replacement)
      else
        new_file_name = String.replace(file, file_placeholder, file_replacement)
        new_path = Path.join(dir, new_file_name)
        File.rename!(old_path, new_path)
      end
    end)
  end

  defp replace_in_files(dir, placeholder, replacement) do
    dir
    |> File.ls!()
    |> Enum.each(fn file ->
      path = Path.join(dir, file)

      if File.dir?(path) do
        replace_in_files(path, placeholder, replacement)
      else
        content = File.read!(path)
        updated_content = String.replace(content, placeholder, replacement)
        File.write!(path, updated_content)
      end
    end)
  end
end
