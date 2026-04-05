export function EmptyState() {
  return (
    <div className="h-screen flex flex-col">
      {/* Top Bar */}
      <div className="h-8 bg-[#0d1117] border-b border-[#1f2937] flex items-center px-4 justify-between">
        <div className="flex items-center gap-4">
          <span className="text-[#39bae6]">◆</span>
          <span className="text-[#e6e6e8]">OxIde</span>
          <span className="text-[#6c7680]">v0.1.0</span>
        </div>
        <div className="flex items-center gap-4 text-xs">
          <span className="text-[#6c7680]">No workspace</span>
          <span className="text-[#6c7680]">│</span>
          <span className="text-[#6c7680]">Terminal: 160×45</span>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex items-center justify-center bg-[#0a0e14]">
        <div className="max-w-3xl w-full px-8">
          {/* Welcome Header */}
          <div className="text-center mb-12">
            <div className="text-6xl text-[#39bae6] mb-4 tracking-tight">OxIde</div>
            <div className="text-[#8a8d92] text-sm">
              Terminal-native IDE for OxVba
            </div>
          </div>

          {/* Quick Actions */}
          <div className="border border-[#1f2937] bg-[#0d1117] mb-8">
            <div className="border-b border-[#1f2937] px-4 py-2 text-sm text-[#8a8d92]">
              Quick Start
            </div>
            <div className="p-6 space-y-3">
              <button className="w-full text-left px-4 py-3 bg-[#1a1f28] hover:bg-[#242933] border border-[#2d3340] transition-colors group">
                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-[#e6e6e8] mb-1">
                      Open Workspace
                    </div>
                    <div className="text-xs text-[#6c7680]">
                      Open an existing .basproj file
                    </div>
                  </div>
                  <kbd className="px-2 py-1 text-xs bg-[#0a0e14] border border-[#2d3340] text-[#8a8d92] group-hover:text-[#39bae6]">
                    Ctrl+O
                  </kbd>
                </div>
              </button>

              <button className="w-full text-left px-4 py-3 bg-[#1a1f28] hover:bg-[#242933] border border-[#2d3340] transition-colors group">
                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-[#e6e6e8] mb-1">
                      Create New Project
                    </div>
                    <div className="text-xs text-[#6c7680]">
                      Initialize a new OxVba project
                    </div>
                  </div>
                  <kbd className="px-2 py-1 text-xs bg-[#0a0e14] border border-[#2d3340] text-[#8a8d92] group-hover:text-[#39bae6]">
                    Ctrl+N
                  </kbd>
                </div>
              </button>

              <button className="w-full text-left px-4 py-3 bg-[#1a1f28] hover:bg-[#242933] border border-[#2d3340] transition-colors group">
                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-[#e6e6e8] mb-1">
                      Open Recent
                    </div>
                    <div className="text-xs text-[#6c7680]">
                      Browse recent workspaces
                    </div>
                  </div>
                  <kbd className="px-2 py-1 text-xs bg-[#0a0e14] border border-[#2d3340] text-[#8a8d92] group-hover:text-[#39bae6]">
                    Ctrl+Shift+O
                  </kbd>
                </div>
              </button>
            </div>
          </div>

          {/* Terminal Setup */}
          <div className="border border-[#1f2937] bg-[#0d1117]">
            <div className="border-b border-[#1f2937] px-4 py-2 flex items-center justify-between">
              <span className="text-sm text-[#8a8d92]">Terminal Setup</span>
              <span className="text-xs text-[#50fa7b]">● Ready</span>
            </div>
            <div className="p-6">
              <div className="grid grid-cols-2 gap-4 text-sm">
                <div className="flex items-center gap-2">
                  <span className="text-[#50fa7b]">✓</span>
                  <span className="text-[#8a8d92]">256 colors</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-[#50fa7b]">✓</span>
                  <span className="text-[#8a8d92]">Unicode support</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-[#50fa7b]">✓</span>
                  <span className="text-[#8a8d92]">Mouse support</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-[#50fa7b]">✓</span>
                  <span className="text-[#8a8d92]">Minimum 160×45</span>
                </div>
              </div>
            </div>
          </div>

          {/* Help Text */}
          <div className="mt-8 text-center text-xs text-[#6c7680]">
            Press <kbd className="px-2 py-0.5 bg-[#1a1f28] border border-[#2d3340] mx-1">Ctrl+P</kbd> for command palette
            <span className="mx-2">•</span>
            Press <kbd className="px-2 py-0.5 bg-[#1a1f28] border border-[#2d3340] mx-1">Alt+H</kbd> for help
          </div>
        </div>
      </div>

      {/* Status Bar */}
      <div className="h-6 bg-[#0d1117] border-t border-[#1f2937] flex items-center px-4 justify-between text-xs">
        <div className="flex items-center gap-4 text-[#6c7680]">
          <span>Ready</span>
        </div>
        <div className="flex items-center gap-4 text-[#6c7680]">
          <span>OxVba 0.4.0</span>
        </div>
      </div>
    </div>
  );
}
