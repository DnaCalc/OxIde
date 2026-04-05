export function MainEditingState() {
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
          <span className="text-[#6c7680]">│</span>
          <span className="text-[#f97e72]">● Unsaved</span>
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 flex overflow-hidden">
        {/* Left: Project Explorer */}
        <div className="w-64 border-r border-[#1f2937] bg-[#0d1117] flex flex-col">
          <div className="h-8 border-b border-[#1f2937] px-3 flex items-center justify-between">
            <span className="text-[#8a8d92] text-xs">EXPLORER</span>
            <div className="flex gap-1">
              <button className="text-[#6c7680] hover:text-[#e6e6e8] text-xs">+</button>
              <button className="text-[#6c7680] hover:text-[#e6e6e8] text-xs">⋯</button>
            </div>
          </div>
          <div className="flex-1 overflow-auto text-sm">
            <div className="px-3 py-2">
              <div className="text-[#8a8d92] text-xs mb-2">PAYROLL.BASPROJ</div>
              
              {/* Modules */}
              <div className="mb-3">
                <div className="text-[#6c7680] text-xs mb-1 flex items-center gap-1">
                  <span>▼</span>
                  <span>Modules</span>
                </div>
                <div className="ml-4 space-y-0.5">
                  <div className="flex items-center gap-2 px-2 py-1 bg-[#1a1f28] border-l-2 border-[#39bae6] text-[#e6e6e8]">
                    <span className="text-[#39bae6]">■</span>
                    <span>Main.bas</span>
                    <span className="ml-auto text-[#f97e72]">●</span>
                  </div>
                  <div className="flex items-center gap-2 px-2 py-1 hover:bg-[#1a1f28] text-[#8a8d92]">
                    <span className="text-[#59c2ff]">■</span>
                    <span>Calculate.bas</span>
                  </div>
                  <div className="flex items-center gap-2 px-2 py-1 hover:bg-[#1a1f28] text-[#8a8d92]">
                    <span className="text-[#59c2ff]">■</span>
                    <span>Reports.bas</span>
                  </div>
                </div>
              </div>

              {/* Forms */}
              <div className="mb-3">
                <div className="text-[#6c7680] text-xs mb-1 flex items-center gap-1">
                  <span>▼</span>
                  <span>Forms</span>
                </div>
                <div className="ml-4 space-y-0.5">
                  <div className="flex items-center gap-2 px-2 py-1 hover:bg-[#1a1f28] text-[#8a8d92]">
                    <span className="text-[#ffb454]">□</span>
                    <span>frmMain.frm</span>
                  </div>
                </div>
              </div>

              {/* References */}
              <div className="mb-3">
                <div className="text-[#6c7680] text-xs mb-1 flex items-center gap-1">
                  <span>▶</span>
                  <span>References</span>
                </div>
              </div>
            </div>
          </div>
          <div className="h-8 border-t border-[#1f2937] px-3 flex items-center text-xs text-[#6c7680]">
            <span>8 items</span>
          </div>
        </div>

        {/* Center: Editor */}
        <div className="flex-1 flex flex-col bg-[#0a0e14]">
          {/* Editor Header */}
          <div className="h-8 border-b border-[#1f2937] px-4 flex items-center justify-between bg-[#0d1117]">
            <div className="flex items-center gap-2">
              <span className="text-[#39bae6]">■</span>
              <span className="text-[#e6e6e8] text-sm">Main.bas</span>
              <span className="text-[#f97e72] ml-2">●</span>
            </div>
            <div className="flex items-center gap-4 text-xs text-[#6c7680]">
              <span>Ln 12, Col 18</span>
              <span>│</span>
              <span>UTF-8</span>
            </div>
          </div>

          {/* Editor Content */}
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
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">4</span>
                <span className="text-[#6c7680]">' Entry point for the application</span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">5</span>
                <span></span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">6</span>
                <span className="text-[#ff79c6]">Public</span>
                <span className="text-[#e6e6e8]"> </span>
                <span className="text-[#ff79c6]">Sub</span>
                <span className="text-[#e6e6e8]"> </span>
                <span className="text-[#ffd580]">Main</span>
                <span className="text-[#e6e6e8]">()</span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">7</span>
                <span className="text-[#e6e6e8]">    </span>
                <span className="text-[#ff79c6]">Dim</span>
                <span className="text-[#e6e6e8]"> employees </span>
                <span className="text-[#ff79c6]">As</span>
                <span className="text-[#e6e6e8]"> </span>
                <span className="text-[#59c2ff]">Collection</span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">8</span>
                <span className="text-[#e6e6e8]">    </span>
                <span className="text-[#ff79c6]">Dim</span>
                <span className="text-[#e6e6e8]"> totalAmount </span>
                <span className="text-[#ff79c6]">As</span>
                <span className="text-[#e6e6e8]"> </span>
                <span className="text-[#59c2ff]">Currency</span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">9</span>
                <span></span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">10</span>
                <span className="text-[#e6e6e8]">    </span>
                <span className="text-[#6c7680]">' Initialize data</span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">11</span>
                <span className="text-[#e6e6e8]">    </span>
                <span className="text-[#ff79c6]">Set</span>
                <span className="text-[#e6e6e8]"> employees = </span>
                <span className="text-[#ff79c6]">New</span>
                <span className="text-[#e6e6e8]"> </span>
                <span className="text-[#59c2ff]">Collection</span>
              </div>
              <div className="flex bg-[#1a1f28] -mx-4 px-4">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">12</span>
                <span className="text-[#e6e6e8]">    totalAmount = </span>
                <span className="text-[#ffd580]">CalculateTo</span>
                <span className="border-l-2 border-[#39bae6] animate-pulse"></span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">13</span>
                <span></span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">14</span>
                <span className="text-[#e6e6e8]">    </span>
                <span className="text-[#6c7680]">' Display results</span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">15</span>
                <span className="text-[#e6e6e8]">    </span>
                <span className="text-[#ffd580]">MsgBox</span>
                <span className="text-[#e6e6e8]"> </span>
                <span className="text-[#c3e88d]">"Total: "</span>
                <span className="text-[#e6e6e8]"> & totalAmount</span>
              </div>
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">16</span>
                <span className="text-[#ff79c6]">End</span>
                <span className="text-[#e6e6e8]"> </span>
                <span className="text-[#ff79c6]">Sub</span>
              </div>
            </div>
          </div>
        </div>

        {/* Right: Inspector */}
        <div className="w-72 border-l border-[#1f2937] bg-[#0d1117] flex flex-col">
          <div className="h-8 border-b border-[#1f2937] px-3 flex items-center">
            <span className="text-[#8a8d92] text-xs">INSPECTOR</span>
          </div>

          {/* Diagnostics */}
          <div className="border-b border-[#1f2937]">
            <div className="px-3 py-2 bg-[#0a0e14]">
              <div className="text-xs text-[#8a8d92] mb-2">DIAGNOSTICS</div>
              <div className="space-y-1 text-xs">
                <div className="flex items-start gap-2">
                  <span className="text-[#50fa7b]">0</span>
                  <span className="text-[#8a8d92]">errors</span>
                </div>
                <div className="flex items-start gap-2">
                  <span className="text-[#ffb454]">1</span>
                  <span className="text-[#8a8d92]">warnings</span>
                </div>
                <div className="flex items-start gap-2">
                  <span className="text-[#59c2ff]">2</span>
                  <span className="text-[#8a8d92]">hints</span>
                </div>
              </div>
            </div>
          </div>

          {/* Symbols */}
          <div className="flex-1 overflow-auto">
            <div className="px-3 py-2">
              <div className="text-xs text-[#8a8d92] mb-2">SYMBOLS</div>
              <div className="space-y-1 text-sm">
                <div className="flex items-center gap-2 text-[#e6e6e8]">
                  <span className="text-[#ff79c6]">ƒ</span>
                  <span>Main</span>
                  <span className="ml-auto text-[#6c7680] text-xs">Ln 6</span>
                </div>
                <div className="ml-4 space-y-1 text-[#8a8d92]">
                  <div className="flex items-center gap-2">
                    <span className="text-[#59c2ff]">ν</span>
                    <span>employees</span>
                    <span className="ml-auto text-[#6c7680] text-xs">Ln 7</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-[#59c2ff]">ν</span>
                    <span>totalAmount</span>
                    <span className="ml-auto text-[#6c7680] text-xs">Ln 8</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Bottom: Utility Surface */}
      <div className="h-48 border-t border-[#1f2937] bg-[#0d1117] flex flex-col">
        {/* Tabs */}
        <div className="h-8 border-b border-[#1f2937] flex items-center px-2 gap-1">
          <button className="px-3 py-1 text-xs bg-[#1a1f28] text-[#e6e6e8] border-b-2 border-[#39bae6]">
            Problems
          </button>
          <button className="px-3 py-1 text-xs text-[#8a8d92] hover:text-[#e6e6e8]">
            Output
          </button>
          <button className="px-3 py-1 text-xs text-[#8a8d92] hover:text-[#e6e6e8]">
            Immediate
          </button>
          <button className="px-3 py-1 text-xs text-[#8a8d92] hover:text-[#e6e6e8]">
            References
          </button>
        </div>

        {/* Problems Content */}
        <div className="flex-1 overflow-auto px-4 py-2 text-sm">
          <div className="space-y-2">
            <div className="flex items-start gap-3 hover:bg-[#1a1f28] px-2 py-1.5 -mx-2">
              <span className="text-[#ffb454]">⚠</span>
              <div className="flex-1">
                <div className="text-[#e6e6e8]">
                  Variable 'totalAmount' is assigned but never used
                </div>
                <div className="text-xs text-[#6c7680] mt-0.5">
                  Main.bas:8:9
                </div>
              </div>
            </div>
            <div className="flex items-start gap-3 hover:bg-[#1a1f28] px-2 py-1.5 -mx-2">
              <span className="text-[#59c2ff]">ℹ</span>
              <div className="flex-1">
                <div className="text-[#e6e6e8]">
                  Consider using Option Strict for type safety
                </div>
                <div className="text-xs text-[#6c7680] mt-0.5">
                  Main.bas:1:1
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Status Bar */}
      <div className="h-6 bg-[#0d1117] border-t border-[#1f2937] flex items-center px-4 justify-between text-xs">
        <div className="flex items-center gap-4 text-[#6c7680]">
          <span className="text-[#50fa7b]">● Edit</span>
          <span>│</span>
          <span>3 buffers</span>
          <span>│</span>
          <span>1 warning, 2 hints</span>
        </div>
        <div className="flex items-center gap-4 text-[#6c7680]">
          <span>OxVba 0.4.0</span>
        </div>
      </div>
    </div>
  );
}
