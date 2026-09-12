export type LineEndingClass = "none" | "lf" | "crlf" | "mixed";

export type ContentSafetyClass = {
  unsafe_html_present: boolean;
  executed: false;
  line_endings: LineEndingClass;
};

export const UNSAFE_HTML_WIKI_CRLF_BODY =
  "<p>欢迎</p>\r\n<script>alert(1)</script>\r\n<img src=x onerror=\"alert(1)\">\r\n参见 [[欢迎]]。\r\n";

export const NATIVE_GUI_IME_UNVERIFIED =
  "native GUI / IME session remains UNVERIFIED; cargo test and npm build are not a typed native window";

export function unsafeHtmlPresent(body: string): boolean {
  const lower = body.toLowerCase();
  return (
    lower.includes("<script") ||
    lower.includes("onerror=") ||
    lower.includes("onload=") ||
    lower.includes("javascript:") ||
    lower.includes("<iframe")
  );
}

export function classifyLineEndings(body: string): LineEndingClass {
  const hasCrlf = body.includes("\r\n");
  const withoutCrlf = body.replaceAll("\r\n", "");
  const hasOther = withoutCrlf.includes("\n") || withoutCrlf.includes("\r");
  if (!hasCrlf && !hasOther) {
    return "none";
  }
  if (hasCrlf && !hasOther) {
    return "crlf";
  }
  if (!hasCrlf && hasOther) {
    return "lf";
  }
  return "mixed";
}

export function classifyBody(body: string): ContentSafetyClass {
  return {
    unsafe_html_present: unsafeHtmlPresent(body),
    executed: false,
    line_endings: classifyLineEndings(body),
  };
}
