export interface InvokeAnalyzeData {
  path_parts: string[];

  extension: string;

  // mp4, png  などのutf-8で解釈できないファイルだった場合-1になる
  char: number;

  // mp4, png  などのutf-8で解釈できないファイルだった場合-1になる
  line: number;

  size: number;
}

export type InvokeAnalyzeResult = InvokeAnalyzeData[];
