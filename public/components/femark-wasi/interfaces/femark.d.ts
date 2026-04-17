export namespace Femark {
  export function processMarkdownToHtml(input: string): HtmlOutput;
}
export interface HtmlOutput {
  toc?: string,
  content: string,
}
export type HighlighterError = HighlighterErrorNolang | HighlighterErrorNohighlighter | HighlighterErrorCouldNotBuildHighlighter | HighlighterErrorStringGenerationError;
export interface HighlighterErrorNolang {
  tag: 'nolang',
}
export interface HighlighterErrorNohighlighter {
  tag: 'nohighlighter',
}
export interface HighlighterErrorCouldNotBuildHighlighter {
  tag: 'could-not-build-highlighter',
  val: string,
}
export interface HighlighterErrorStringGenerationError {
  tag: 'string-generation-error',
  val: string,
}
