class Astra < Formula
  desc "A modern Rust-based shell"
  homepage "https://github.com/SYOP200/astra-shell"
  url "https://github.com/SYOP200/astra-shell/archive/refs/tags/v0.4.0.tar.gz"
  version "0.4.0"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"

    bin.install "target/release/astra"
  end

  test do
    assert_match "astra", shell_output("#{bin}/astra --version")
  end
end
