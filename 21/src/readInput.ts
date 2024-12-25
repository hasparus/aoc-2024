export async function readInput(name: string): Promise<string> {
  const path = new URL("../inputs.md", import.meta.url);

  const file = Bun.file(path);
  const text = await file.text();

  const indexOfHeading = text.indexOf(name);
  const indexOfCodeBlock = text.indexOf("```", indexOfHeading);
  const codeBlock = text.slice(
    indexOfCodeBlock + 3,
    text.indexOf("```", indexOfCodeBlock + 3)
  );

  return codeBlock.trim();
}
