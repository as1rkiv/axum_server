use regex::Regex;

// 被边框包裹的打印消息
pub struct BoxedMessage<'a> {
    contents: Vec<&'a str>,
    sign: &'a str,
    margin_tb: usize,
    margin_left: usize,
    padding_tb: usize,
    padding_lr: usize,
}

impl<'a> BoxedMessage<'a> {
    pub fn new(sign: &'a str) -> Self {
        Self {
            contents: vec![],
            sign,
            margin_tb: 0,
            margin_left: 0,
            padding_tb: 0,
            padding_lr: 0,
        }
    }

    // 设置外边距
    pub fn set_margin(mut self, tb: usize, left: usize) -> Self {
        self.margin_tb = tb;
        self.margin_left = left;
        self
    }

    // 设置内边距
    pub fn set_padding(mut self, tb: usize, lr: usize) -> Self {
        self.padding_tb = tb;
        self.padding_lr = lr;
        self
    }

    // 添加内容
    pub fn append(mut self, content: &'a str) -> Self {
        self.contents.push(content);
        self
    }

    // 去除 ANSI 转义码
    fn strip_ansi_codes(s: &str) -> String {
        if let Ok(re) = Regex::new(r#"\x1b\[[0-9;]*m"#) {
            return re.replace_all(s, r#""#).to_string();
        };

        s.to_string()
    }

    // 计算字符串宽度（考虑中英文字符的宽度差异）
    fn calculate_width(s: &str) -> usize {
        s.chars()
            .map(|c| {
                if c.is_ascii() {
                    1 // 英文字符
                } else {
                    2 // 中文字符
                }
            })
            .sum()
    }

    // 打印方框
    pub fn show_message(&self) {
        let border_char = self.sign;
        let padding_char = ' ';

        // 左侧外边距（缩进）
        let indent = r#" "#.repeat(self.margin_left);

        // 打印上部外边距
        for _ in 0..self.margin_tb {
            println!();
        }

        // 计算最长内容的实际长度（去除颜色代码）
        let content_max_width = self
            .contents
            .iter()
            .map(|s| Self::calculate_width(&Self::strip_ansi_codes(s)))
            .max()
            .unwrap_or(0);

        // 计算内容宽度
        let content_width = self.padding_lr + content_max_width;
        let full_width = content_width + 2; // +2 for border chars

        // 打印顶部边框
        println!(r#"{}{}"#, indent, border_char.repeat(full_width));

        // 打印顶部内边距
        for _ in 0..self.padding_tb {
            println!(
                r#"{}{}{}{}"#,
                indent,
                border_char,
                r#" "#.repeat(content_width),
                border_char
            );
        }

        // 打印内容行
        for message in &self.contents {
            let stripped_message = Self::strip_ansi_codes(message);
            let stripped_width = Self::calculate_width(&stripped_message);
            let padding = if stripped_width < content_width {
                (content_width - stripped_width) / 2
            } else {
                0
            };
            let left_padding_str = padding_char.to_string().repeat(padding);
            let right_padding_str = padding_char
                .to_string()
                .repeat(content_width - padding - stripped_width);

            // 打印内容，支持带颜色
            println!(
                r#"{}{}{}{}{}{}"#,
                indent, border_char, left_padding_str, message, right_padding_str, border_char
            );
        }

        // 打印底部内边距
        for _ in 0..self.padding_tb {
            println!(
                r#"{}{}{}{}"#,
                indent,
                border_char,
                r#" "#.repeat(content_width),
                border_char
            );
        }

        // 打印底部边框
        println!(r#"{}{}"#, indent, border_char.repeat(full_width));

        // 打印下部外边距
        for _ in 0..self.margin_tb {
            println!();
        }
    }
}
