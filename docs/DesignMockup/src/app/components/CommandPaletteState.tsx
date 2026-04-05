export function CommandPaletteState() {
  return (
    <div className="h-screen flex flex-col">
      {/* Top Bar */}
      <div className="h-8 bg-[#0d1117] border-b border-[#1f2937] flex items-center px-4 justify-between text-xs">
        <div className="flex items-center gap-4">
          <span className="text-[#39bae6]">◆</span>
          <span className="text-[#e6e6e8]">Payroll.basproj</span>
          <span className="text-[#6c7680]">│</span>
          <span className="text-[#8a8d92]">Target:</span>
          <span className="text-[#e6e6e8]">Exe</span>
          <span className="text-[#6c7680]">│</span>
          <span className="text-[#8a8d92]">Config:</span>
          <span className="text-[#e6e6e8]">win-console</span>
        </div>
        <div className="flex items-center gap-4">
          <span className="text-[#6c7680]">Buffer: 1 of 3</span>
        </div>
      </div>

      {/* Main Content Area - Dimmed */}
      <div className="flex-1 flex overflow-hidden relative">
        {/* Left: Project Explorer - Dimmed */}
        <div className="w-64 border-r border-[#1f2937] bg-[#0d1117] flex flex-col opacity-40">
          <div className="h-8 border-b border-[#1f2937] px-3 flex items-center justify-between">
            <span className="text-[#8a8d92] text-xs">EXPLORER</span>
            <div className="flex gap-1">
              <button className="text-[#6c7680] text-xs">+</button>
              <button className="text-[#6c7680] text-xs">⋯</button>
            </div>
          </div>
          <div className="flex-1 overflow-auto text-sm">
            <div className="px-3 py-2">
              <div className="text-[#8a8d92] text-xs mb-2">PAYROLL.BASPROJ</div>
              
              <div className="mb-3">
                <div className="text-[#6c7680] text-xs mb-1 flex items-center gap-1">
                  <span>▼</span>
                  <span>Modules</span>
                </div>
                <div className="ml-4 space-y-0.5">
                  <div className="flex items-center gap-2 px-2 py-1 bg-[#1a1f28] text-[#e6e6e8]">
                    <span className="text-[#39bae6]">■</span>
                    <span>Main.bas</span>
                  </div>
                  <div className="flex items-center gap-2 px-2 py-1 text-[#8a8d92]">
                    <span className="text-[#59c2ff]">■</span>
                    <span>Calculate.bas</span>
                  </div>
                  <div className="flex items-center gap-2 px-2 py-1 text-[#8a8d92]">
                    <span className="text-[#59c2ff]">■</span>
                    <span>Reports.bas</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Center: Editor - Dimmed */}
        <div className="flex-1 flex flex-col bg-[#0a0e14] opacity-40">
          <div className="h-8 border-b border-[#1f2937] px-4 flex items-center justify-between bg-[#0d1117]">
            <div className="flex items-center gap-2">
              <span className="text-[#39bae6]">■</span>
              <span className="text-[#e6e6e8] text-sm">Main.bas</span>
            </div>
          </div>

          <div className="flex-1 overflow-auto px-4 py-3">
            <div className="space-y-0 leading-relaxed">
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">1</span>
                <span className="text-[#ff79c6]">Option</span>
                <span className="text-[#e6e6e8]"> </span>
                <span className="text-[#ff79c6]">Explicit</span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">2</span>
                <span></span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">3</span>
                <span className="text-[#6c7680]">' Payroll calculation module</span>
              </div>
            </div>
          </div>
        </div>

        {/* Right: Inspector - Dimmed */}
        <div className="w-72 border-l border-[#1f2937] bg-[#0d1117] flex flex-col opacity-40">
          <div className="h-8 border-b border-[#1f2937] px-3 flex items-center">
            <span className="text-[#8a8d92] text-xs">INSPECTOR</span>
          </div>
        </div>

        {/* Command Palette Overlay */}
        <div className="absolute inset-0 flex items-start justify-center pt-32 bg-black/50">
          <div className="w-[600px] border-2 border-[#39bae6] bg-[#0d1117] shadow-2xl">
            {/* Palette Header */}
            <div className="border-b border-[#1f2937] px-4 py-3 bg-[#1a1f28]">
              <div className="flex items-center gap-2 text-sm">
                <span className="text-[#39bae6]">›</span>
                <input
                  type="text"
                  placeholder="Type a command or search..."
                  className="flex-1 bg-transparent text-[#e6e6e8] outline-none placeholder:text-[#6c7680]"
                  autoFocus
                  defaultValue="inse"
                />
                <kbd className="px-2 py-0.5 text-xs bg-[#0a0e14] border border-[#2d3340] text-[#6c7680]">
                  Esc
                </kbd>
              </div>
            </div>

            {/* Results */}
            <div className="max-h-96 overflow-auto">
              {/* Command Category */}
              <div className="px-3 py-2 bg-[#0a0e14]">
                <div className="text-xs text-[#8a8d92]">COMMANDS</div>
              </div>

              {/* Command Items */}
              <div className="py-1">
                <button className="w-full px-4 py-2.5 text-left hover:bg-[#1a1f28] flex items-center justify-between group bg-[#1a1f28] border-l-2 border-[#39bae6]">
                  <div className="flex items-center gap-3">
                    <span className="text-[#ffb454]">+</span>
                    <div>
                      <div className="text-sm text-[#e6e6e8] mb-0.5">
                        <span className="text-[#39bae6]">Inse</span>rt Module
                      </div>
                      <div className="text-xs text-[#6c7680]">
                        Add a new module to the project
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <kbd className="px-2 py-0.5 text-xs bg-[#0a0e14] border border-[#2d3340] text-[#6c7680]">
                      Alt+I
                    </kbd>
                    <kbd className="px-2 py-0.5 text-xs bg-[#0a0e14] border border-[#2d3340] text-[#6c7680]">
                      M
                    </kbd>
                  </div>
                </button>

                <button className="w-full px-4 py-2.5 text-left hover:bg-[#1a1f28] flex items-center justify-between group">
                  <div className="flex items-center gap-3">
                    <span className="text-[#ffb454]">+</span>
                    <div>
                      <div className="text-sm text-[#e6e6e8] mb-0.5">
                        <span className="text-[#39bae6]">Inse</span>rt Class Module
                      </div>
                      <div className="text-xs text-[#6c7680]">
                        Add a new class module to the project
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <kbd className="px-2 py-0.5 text-xs bg-[#0a0e14] border border-[#2d3340] text-[#6c7680]">
                      Alt+I
                    </kbd>
                    <kbd className="px-2 py-0.5 text-xs bg-[#0a0e14] border border-[#2d3340] text-[#6c7680]">
                      C
                    </kbd>
                  </div>
                </button>

                <button className="w-full px-4 py-2.5 text-left hover:bg-[#1a1f28] flex items-center justify-between group">
                  <div className="flex items-center gap-3">
                    <span className="text-[#ffb454]">+</span>
                    <div>
                      <div className="text-sm text-[#e6e6e8] mb-0.5">
                        <span className="text-[#39bae6]">Inse</span>rt Form
                      </div>
                      <div className="text-xs text-[#6c7680]">
                        Add a new form to the project
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <kbd className="px-2 py-0.5 text-xs bg-[#0a0e14] border border-[#2d3340] text-[#6c7680]">
                      Alt+I
                    </kbd>
                    <kbd className="px-2 py-0.5 text-xs bg-[#0a0e14] border border-[#2d3340] text-[#6c7680]">
                      F
                    </kbd>
                  </div>
                </button>
              </div>

              {/* Symbol Category */}
              <div className="px-3 py-2 bg-[#0a0e14] border-t border-[#1f2937]">
                <div className="text-xs text-[#8a8d92]">SYMBOLS</div>
              </div>

              <div className="py-1">
                <button className="w-full px-4 py-2.5 text-left hover:bg-[#1a1f28] flex items-center justify-between group">
                  <div className="flex items-center gap-3">
                    <span className="text-[#ff79c6]">ƒ</span>
                    <div>
                      <div className="text-sm text-[#e6e6e8]">
                        <span className="text-[#39bae6]">Inse</span>rtEmployee
                      </div>
                      <div className="text-xs text-[#6c7680]">
                        Calculate.bas:42
                      </div>
                    </div>
                  </div>
                </button>

                <button className="w-full px-4 py-2.5 text-left hover:bg-[#1a1f28] flex items-center justify-between group">
                  <div className="flex items-center gap-3">
                    <span className="text-[#ff79c6]">ƒ</span>
                    <div>
                      <div className="text-sm text-[#e6e6e8]">
                        Update<span className="text-[#39bae6]">Inse</span>rtionPoint
                      </div>
                      <div className="text-xs text-[#6c7680]">
                        Reports.bas:18
                      </div>
                    </div>
                  </div>
                </button>
              </div>

              {/* File Category */}
              <div className="px-3 py-2 bg-[#0a0e14] border-t border-[#1f2937]">
                <div className="text-xs text-[#8a8d92]">FILES</div>
              </div>

              <div className="py-1">
                <button className="w-full px-4 py-2.5 text-left hover:bg-[#1a1f28] flex items-center justify-between group">
                  <div className="flex items-center gap-3">
                    <span className="text-[#59c2ff]">■</span>
                    <div>
                      <div className="text-sm text-[#e6e6e8]">
                        Main.bas
                      </div>
                      <div className="text-xs text-[#6c7680]">
                        Modules/Main.bas
                      </div>
                    </div>
                  </div>
                </button>
              </div>
            </div>

            {/* Footer */}
            <div className="border-t border-[#1f2937] px-4 py-2 bg-[#0a0e14] flex items-center justify-between text-xs">
              <div className="flex items-center gap-4 text-[#6c7680]">
                <span>8 results</span>
                <span>│</span>
                <div className="flex items-center gap-2">
                  <kbd className="px-1.5 py-0.5 bg-[#1a1f28] border border-[#2d3340]">↑↓</kbd>
                  <span>navigate</span>
                </div>
                <div className="flex items-center gap-2">
                  <kbd className="px-1.5 py-0.5 bg-[#1a1f28] border border-[#2d3340]">Enter</kbd>
                  <span>select</span>
                </div>
              </div>
              <div className="text-[#6c7680]">
                Ctrl+P
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Bottom: Utility Surface - Dimmed */}
      <div className="h-48 border-t border-[#1f2937] bg-[#0d1117] flex flex-col opacity-40">
        <div className="h-8 border-b border-[#1f2937] flex items-center px-2 gap-1">
          <button className="px-3 py-1 text-xs bg-[#1a1f28] text-[#e6e6e8]">
            Problems
          </button>
        </div>
      </div>

      {/* Status Bar */}
      <div className="h-6 bg-[#0d1117] border-t border-[#1f2937] flex items-center px-4 justify-between text-xs">
        <div className="flex items-center gap-4 text-[#6c7680]">
          <span className="text-[#50fa7b]">● Edit</span>
          <span>│</span>
          <span>Command Palette Active</span>
        </div>
        <div className="flex items-center gap-4 text-[#6c7680]">
          <span>OxVba 0.4.0</span>
        </div>
      </div>
    </div>
  );
}
