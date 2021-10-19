#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn test_not_pass() {
    assert!(!judge_message_lint_pass("demo/demo.toml", "fix"));
    assert!(!judge_message_lint_pass("demo/demo.toml", "feat:not scope"));
    assert!(!judge_message_lint_pass("demo/demo.toml", "notype(scope):not scope"));
    assert!(!judge_message_lint_pass("demo/demo.toml", "feat(client):fix"));
    assert!(!judge_message_lint_pass("demo/demo.toml", "feat(client):中文"));
    assert!(!judge_message_lint_pass("demo/demo.toml", "修复一个问题"));
  }

  #[test]
  fn test_pass() {
    assert!(judge_message_lint_pass("demo/demo.toml", "feat(all): add a test"));
    assert!(judge_message_lint_pass("demo/demo.toml", "feat(jupyter): update server path"));
    assert!(judge_message_lint_pass("demo/demo.toml", "fix(common): 中文测试"));
    assert!(judge_message_lint_pass("demo/demo.toml", "chore(common): 中文测试"));
    assert!(judge_message_lint_pass("demo/demo.toml", "chore(common,jupyter): 中文测试"));
    assert!(judge_message_lint_pass("demo/demo.toml", "fix(all,common,jupyter): 中文测试"));
    assert!(judge_message_lint_pass("demo/demo.toml", "fix(all,common,jupyter): 中文测试\n翻页"));
    assert!(judge_message_lint_pass("demo/demo.toml", "fix(all,common,jupyter): 中文测试
       直接换行"));
    assert!(judge_message_lint_pass("demo/demo.toml", "Merge branch 'wentao-dev'"));
  }
}