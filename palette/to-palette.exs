defmodule CrossStitch do
  def from_json(json_path, palette_path) do
    json = File.read!(json_path)
    data = Jason.decode!(json)

    palette_data =
      data
      |> Enum.sort_by(&code/1)
      |> Enum.map(&to_palette_line/1)
      |> Enum.join("\n")

    header = """
    GIMP Palette
    Name: DMC Floss Colors
    Columns: 5

    """

    contents = header <> palette_data

    File.write(palette_path, contents, [:write])
  end

  def to_palette_line(floss) do
    "#{floss["color"]["r"]}\t#{floss["color"]["g"]}\t#{floss["color"]["b"]}\t#{floss["code"]}"
  end

  def code(floss) do
    case Integer.parse(floss["code"]) do
      {int, _} -> int
      :error -> floss["code"]
    end
  end
end

CrossStitch.from_json('../data/dmc-colors-rgb.json', '../data/dmc-gimp-palette.gpl')
