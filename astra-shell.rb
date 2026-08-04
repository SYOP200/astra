class AstraShell < Formula
  desc "A modern interactive shell for macOS and Unix"
  homepage "https://github.com/astra-shell/astra-shell"
  url "https://github.com/astra-shell/astra-shell/archive/refs/tags/v1.0.0.tar.gz"
  version "1.0.0"
  sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
  license "MIT"
  head "https://github.com/astra-shell/astra-shell.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "Astra", shell_output("#{bin}/astra --version")
  end
end
