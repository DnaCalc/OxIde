export function BuildRunState() {
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
          <span className="text-[#50fa7b]">● Running</span>
          <span className="text-[#6c7680]">│</span>
          <span className="text-[#8a8d92]">PID: 12845</span>
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
              
              <div className="mb-3">
                <div className="text-[#6c7680] text-xs mb-1 flex items-center gap-1">
                  <span>▼</span>
                  <span>Modules</span>
                </div>
                <div className="ml-4 space-y-0.5">
                  <div className="flex items-center gap-2 px-2 py-1 bg-[#1a1f28] border-l-2 border-[#39bae6] text-[#e6e6e8]">
                    <span className="text-[#39bae6]">■</span>
                    <span>Main.bas</span>
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
            </div>
            <div className="flex items-center gap-4 text-xs text-[#6c7680]">
              <span>Ln 6, Col 1</span>
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
              <div className="flex bg-[#1a1f28] -mx-4 px-4 border-l-4 border-[#50fa7b]">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">6</span>
                <span className="text-[#ff79c6]">Public</span>
                <span className="text-[#e6e6e8]"> </span>
                <span className="text-[#ff79c6]">Sub</span>
                <span className="text-[#e6e6e8]"> </span>
                <span className="text-[#ffd580]">Main</span>
                <span className="text-[#e6e6e8]">()</span>
                <span className="ml-4 text-[#50fa7b] text-xs">← Current execution point</span>
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
              <div className="flex">
                <span className="w-12 text-right text-[#3e4451] pr-4 select-none">12</span>
                <span className="text-[#e6e6e8]">    totalAmount = </span>
                <span className="text-[#ffd580]">CalculateTotal</span>
                <span className="text-[#e6e6e8]">(employees)</span>
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

        {/* Right: Run Status */}
        <div className="w-72 border-l border-[#1f2937] bg-[#0d1117] flex flex-col">
          <div className="h-8 border-b border-[#1f2937] px-3 flex items-center justify-between">
            <span className="text-[#8a8d92] text-xs">RUN STATUS</span>
            <button className="text-[#f97e72] hover:text-[#ff5555] text-xs">■ Stop</button>
          </div>

          {/* Build Info */}
          <div className="border-b border-[#1f2937] px-3 py-3">
            <div className="text-xs text-[#8a8d92] mb-2">BUILD</div>
            <div className="space-y-1.5 text-xs">
              <div className="flex items-center justify-between">
                <span className="text-[#8a8d92]">Status</span>
                <span className="text-[#50fa7b]">Success</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-[#8a8d92]">Duration</span>
                <span className="text-[#e6e6e8]">1.2s</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-[#8a8d92]">Built at</span>
                <span className="text-[#e6e6e8]">14:32:18</span>
              </div>
            </div>
          </div>

          {/* Runtime Info */}
          <div className="border-b border-[#1f2937] px-3 py-3">
            <div className="text-xs text-[#8a8d92] mb-2">RUNTIME</div>
            <div className="space-y-1.5 text-xs">
              <div className="flex items-center justify-between">
                <span className="text-[#8a8d92]">State</span>
                <span className="text-[#50fa7b]">● Running</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-[#8a8d92]">PID</span>
                <span className="text-[#e6e6e8]">12845</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-[#8a8d92]">Uptime</span>
                <span className="text-[#e6e6e8]">0:03:42</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-[#8a8d92]">Memory</span>
                <span className="text-[#e6e6e8]">12.4 MB</span>
              </div>
            </div>
          </div>

          {/* Actions */}
          <div className="px-3 py-3">
            <div className="text-xs text-[#8a8d92] mb-2">ACTIONS</div>
            <div className="space-y-1">
              <button className="w-full px-3 py-2 bg-[#1a1f28] hover:bg-[#242933] border border-[#2d3340] text-left text-sm text-[#e6e6e8]">
                <div className="flex items-center justify-between">
                  <span>Pause</span>
                  <kbd className="text-xs text-[#6c7680]">F8</kbd>
                </div>
              </button>
              <button className="w-full px-3 py-2 bg-[#1a1f28] hover:bg-[#242933] border border-[#2d3340] text-left text-sm text-[#e6e6e8]">
                <div className="flex items-center justify-between">
                  <span>Restart</span>
                  <kbd className="text-xs text-[#6c7680]">Ctrl+Shift+F5</kbd>
                </div>
              </button>
              <button className="w-full px-3 py-2 bg-[#2d1717] hover:bg-[#3d1f1f] border border-[#4d2828] text-left text-sm text-[#f97e72]">
                <div className="flex items-center justify-between">
                  <span>Stop</span>
                  <kbd className="text-xs text-[#6c7680]">Shift+F5</kbd>
                </div>
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Bottom: Output */}
      <div className="h-64 border-t border-[#1f2937] bg-[#0d1117] flex flex-col">
        {/* Tabs */}
        <div className="h-8 border-b border-[#1f2937] flex items-center px-2 gap-1">
          <button className="px-3 py-1 text-xs bg-[#1a1f28] text-[#e6e6e8] border-b-2 border-[#39bae6]">
            Output
          </button>
          <button className="px-3 py-1 text-xs text-[#8a8d92] hover:text-[#e6e6e8]">
            Build Log
          </button>
          <button className="px-3 py-1 text-xs text-[#8a8d92] hover:text-[#e6e6e8]">
            Immediate
          </button>
          <button className="px-3 py-1 text-xs text-[#8a8d92] hover:text-[#e6e6e8]">
            Problems
          </button>
        </div>

        {/* Output Content */}
        <div className="flex-1 overflow-auto px-4 py-2 text-sm font-mono">
          <div className="space-y-1">
            <div className="text-[#6c7680]">
              [14:32:17] Building Payroll.basproj...
            </div>
            <div className="text-[#59c2ff]">
              [14:32:17] → Compiling Main.bas
            </div>
            <div className="text-[#59c2ff]">
              [14:32:17] → Compiling Calculate.bas
            </div>
            <div className="text-[#59c2ff]">
              [14:32:18] → Compiling Reports.bas
            </div>
            <div className="text-[#50fa7b]">
              [14:32:18] ✓ Build succeeded in 1.2s
            </div>
            <div className="text-[#6c7680]">
              [14:32:18] Starting Payroll.exe...
            </div>
            <div className="text-[#50fa7b]">
              [14:32:18] ✓ Process started (PID: 12845)
            </div>
            <div className="text-[#6c7680] mt-2">
              ════════════════════════════════════════════════════
            </div>
            <div className="text-[#e6e6e8] mt-2">
              Payroll System v1.0
            </div>
            <div className="text-[#e6e6e8]">
              Initializing...
            </div>
            <div className="text-[#e6e6e8]">
              Loading employee data...
            </div>
            <div className="text-[#50fa7b]">
              Processing payroll for 127 employees
            </div>
            <div className="text-[#e6e6e8]">
              Calculating taxes...
            </div>
            <div className="text-[#e6e6e8] flex items-center gap-2">
              <span>Working</span>
              <span className="animate-pulse">...</span>
            </div>
          </div>
        </div>
      </div>

      {/* Status Bar */}
      <div className="h-6 bg-[#0d1117] border-t border-[#1f2937] flex items-center px-4 justify-between text-xs">
        <div className="flex items-center gap-4 text-[#6c7680]">
          <span className="text-[#50fa7b]">● Running</span>
          <span>│</span>
          <span>PID: 12845</span>
          <span>│</span>
          <span>Uptime: 0:03:42</span>
        </div>
        <div className="flex items-center gap-4 text-[#6c7680]">
          <span>OxVba 0.4.0</span>
        </div>
      </div>
    </div>
  );
}
