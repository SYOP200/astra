class AstraShell < Formula
  desc "A modern interactive shell for macOS and Unix"
  homepage "https://github.com/astra-shell/astra-shell"
  url "https://github.com/astra-shell/astra-shell/archive/refs/tags/v1.0.0.tar.gz"
  version "1.0.0"
  sha256 "4856e074fc097c3e356b826dc3998fef6f988c9158122ed6c3e5b41d2348695b"
  license "MIT"
  head "https://github.com/astra-shell/astra-shell.git", branch: "main"

  depends_on "rust" => :build
  depends_on "eza"

  def install
    ohai "Installing Astra Shell..."
    system "cargo", "install", *std_cargo_args
  end

  def caveats
    <<~EOS
      Astra Shell has been installed.
      Run `astra` to start the shell.
      Uninstall with `brew uninstall astra-shell`.
    EOS
  end

  test do
    assert_match "Astra", shell_output("#{bin}/astra --version")
  end
end
