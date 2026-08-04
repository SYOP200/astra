class AstraShell < Formula
  desc "A modern interactive shell for macOS and Unix"
  homepage "https://github.com/astra-shell/astra-shell"
  url "https://github.com/astra-shell/astra-shell/archive/refs/tags/v0.4.0.tar.gz"
  sha256 :no_check
  license "MIT"
  head "https://github.com/astra-shell/astra-shell.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_predicate bin/"astra", :exist?
  end
end
