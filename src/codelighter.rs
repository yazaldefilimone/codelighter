#[derive(Debug, Clone, Copy, Default)]
struct Line {
  number: usize,
  start_index: usize,
  end_index: usize,
}

const COLOR_ERR: &str = "\x1b[4m\x1b[31m";
const COLOR_WARN: &str = "\x1b[4m\x1b[33m";
const COLOR_INFO: &str = "\x1b[4m\x1b[36m";
const RESET: &str = "\x1b[0m";

const DEFAULT_CONTEXT: usize = 2;

pub fn highlight_error(start: usize, end: usize, raw: &str) -> String {
  highlight(start, end, raw, COLOR_ERR, DEFAULT_CONTEXT)
}

pub fn highlight_warn(start: usize, end: usize, raw: &str) -> String {
  highlight(start, end, raw, COLOR_WARN, DEFAULT_CONTEXT)
}

pub fn highlight_note(start: usize, end: usize, raw: &str) -> String {
  highlight(start, end, raw, COLOR_INFO, DEFAULT_CONTEXT)
}

pub fn highlight(start: usize, end: usize, raw: &str, color: &str, ctx_lines: usize) -> String {
  assert!(start <= end);
  let owned;
  let text: &str = if end <= raw.len() {
    raw
  } else {
    owned = format!("{}{}", raw, " ".repeat(end - raw.len()));
    &owned
  };

  let mut line_offsets = Vec::new();
  line_offsets.push(0);
  for (i, c) in text.char_indices() {
    if c == '\n' {
      line_offsets.push(i + 1);
    }
  }

  let find_line = |pos: usize| -> usize {
    match line_offsets.binary_search(&pos) {
      Ok(i) => i,
      Err(i) => i.saturating_sub(1),
    }
  };

  let start_line = find_line(start);
  let mut end_line = if end > start {
    find_line(end.saturating_sub(1))
  } else {
    start_line
  };

  if end_line < start_line {
    end_line = start_line;
  }

  let last_line_index = line_offsets.len().saturating_sub(1);
  let context_start = start_line.saturating_sub(ctx_lines);
  let context_end = (end_line + ctx_lines).min(last_line_index);

  let lines: Vec<Line> = (context_start..=context_end)
    .map(|i| {
      let start_idx = line_offsets[i];
      let end_idx = *line_offsets.get(i + 1).unwrap_or(&text.len());
      Line { number: i + 1, start_index: start_idx, end_index: end_idx }
    })
    .collect();

  let max_line_num = lines.last().map(|l| l.number).unwrap_or(1);
  let number_width = max_line_num.to_string().len();

  let mut result = String::with_capacity(text.len() * 2);
  let mut highlighting = false;

  for line in &lines {
    let line_num_str = format!("{:>width$}", line.number, width = number_width);
    result.push_str(&line_num_str);
    result.push_str(" | ");

    let mut current_index = line.start_index;
    for chr in text[line.start_index..line.end_index].chars() {
      let is_in_highlight = current_index >= start && current_index < end;
      if is_in_highlight && !highlighting {
        result.push_str(color);
        highlighting = true;
      } else if !is_in_highlight && highlighting {
        result.push_str(RESET);
        highlighting = false;
      }

      result.push(chr);
      current_index += chr.len_utf8();
    }

    if highlighting {
      result.push_str(RESET);
      highlighting = false;
    }
  }

  result
}
