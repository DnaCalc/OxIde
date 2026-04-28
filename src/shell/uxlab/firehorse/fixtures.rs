use super::projection::*;

pub fn projection_for_scenario(id: &str) -> Option<FireHorseProjection> {
    match id {
        "firehorse-launchpad-standard" => Some(launchpad()),
        "firehorse-editing-lens-standard" => Some(editing_lens()),
        "firehorse-command-lens-standard" => Some(command_lens()),
        "firehorse-run-lane-standard" => Some(run_lane()),
        "firehorse-debug-cockpit-standard" => Some(debug_cockpit()),
        "firehorse-console-fit-light" => Some(console_fit()),
        "firehorse-focus-compact" => Some(focus_compact()),
        _ => None,
    }
}

fn launchpad() -> FireHorseProjection {
    let layout = LayoutPosture::Launchpad;
    FireHorseProjection {
        scenario_id: "firehorse-launchpad-standard",
        expected_layout: layout,
        identity: identity(layout, "No project", None, None),
        project_spine: None,
        code_canvas: welcome_canvas(),
        context_dock: Some(ContextDockProjection {
            title: "Start Context".to_string(),
            cards: vec![ContextCardProjection::Unavailable(UnavailableProjection {
                source: "ProjectSession",
                reason: "No project is mounted in Launchpad.",
            })],
        }),
        activity_deck: ActivityDeckProjection {
            posture: DeckPosture::Rail,
            active: ActivityKind::Output,
            tabs: tabs(&[(ActivityKind::Output, "Recent", Some(3))]),
            rows: vec![
                ActivityRowProjection::Text {
                    source: "SessionStore::recent_workspaces",
                    text: "NorthwindPricing | ExcelDesktop | healthy | 2h ago".to_string(),
                },
                ActivityRowProjection::Text {
                    source: "SessionStore::recent_workspaces",
                    text: "RibbonLab | ExcelAddIn | warning | yesterday".to_string(),
                },
            ],
        },
        key_rail: key_rail(&[
            action("Open Project", "Ctrl+O", "project.open"),
            action("Create Project", "Ctrl+N", "project.create"),
            action("Import", "Ctrl+Shift+O", "project.import"),
            action("Console Fit", "F10", "app.console_fit"),
            action("Quit", "Ctrl+Q", "app.quit"),
        ]),
        overlay: None,
        theme: ThemeProjection::GraphiteEmber,
        terminal_fit: None,
        layout,
        seams: SeamFixtureSet::empty(),
    }
}

fn editing_lens() -> FireHorseProjection {
    let layout = LayoutPosture::Editing;
    let diagnostics = vec![pricing_diagnostic()];
    let symbols = vec![price_for_symbol()];
    let lens = SourceLensProjection {
        anchor: range(6, 12, 6, 20),
        title: "Function PriceFor(productId As Long) As Currency".to_string(),
        body: vec![
            "OxVba hover: resolves to pricing module function.".to_string(),
            "Provenance: HostWorkspaceSession::hover".to_string(),
        ],
        actions: vec![
            action("Go Definition", "F12", "semantic.goto_definition"),
            action("References", "Shift+F12", "semantic.references"),
        ],
        source: SeamSourceProjection {
            provider: "HostWorkspaceSession::hover",
            query: "hover(doc://NorthwindPricing/PriceFor.bas:6:12)",
        },
    };

    FireHorseProjection {
        scenario_id: "firehorse-editing-lens-standard",
        expected_layout: layout,
        identity: identity(
            layout,
            "NorthwindPricing",
            Some("ExcelDesktop"),
            Some(CursorProjection {
                line: 6,
                column: 18,
            }),
        ),
        project_spine: Some(project_spine()),
        code_canvas: CodeCanvasProjection {
            document_label: "PriceFor.bas".to_string(),
            language: "VBA",
            lines: pricing_source_lines(),
            lens: Some(lens),
            execution_line: None,
            selection: Some(range(6, 12, 6, 20)),
        },
        context_dock: Some(ContextDockProjection {
            title: "Context Dock".to_string(),
            cards: vec![
                ContextCardProjection::Diagnostic(diagnostics[0].clone()),
                ContextCardProjection::Symbol(symbols[0].clone()),
            ],
        }),
        activity_deck: ActivityDeckProjection {
            posture: DeckPosture::Expanded,
            active: ActivityKind::Problems,
            tabs: tabs(&[
                (ActivityKind::Problems, "Problems", Some(1)),
                (ActivityKind::Output, "Output", Some(0)),
                (ActivityKind::References, "References", Some(4)),
            ]),
            rows: vec![ActivityRowProjection::Diagnostic(diagnostics[0].clone())],
        },
        key_rail: key_rail(&[
            action("Save", "Ctrl+S", "editor.save"),
            action("Command Lens", "F6", "command.lens.open"),
            action("Semantic Lens", "F1", "semantic.hover"),
            action("Run", "F5", "run.start"),
            action("Quick Fix", "Ctrl+.", "diagnostic.quick_fix"),
        ]),
        overlay: None,
        theme: ThemeProjection::GraphiteEmber,
        terminal_fit: None,
        layout,
        seams: SeamFixtureSet {
            diagnostics,
            symbols,
            ..SeamFixtureSet::empty()
        },
    }
}

fn command_lens() -> FireHorseProjection {
    let layout = LayoutPosture::CommandLens;
    let mut projection = editing_lens();
    projection.scenario_id = "firehorse-command-lens-standard";
    projection.expected_layout = layout;
    projection.layout = layout;
    projection.identity.scene = layout;
    projection.overlay = Some(OverlayProjection::CommandLens(CommandLensProjection {
        filter: "run".to_string(),
        selected_action_id: "run.start",
        rows: vec![
            command_row("run.start", "Run Project", Some("F5"), true, None),
            command_row(
                "run.stop",
                "Stop Run",
                Some("F8"),
                false,
                Some("No active run"),
            ),
            command_row(
                "target.configure",
                "Configure Target",
                Some("Ctrl+K T"),
                true,
                None,
            ),
            command_row(
                "target.switch",
                "Switch Target",
                Some("Ctrl+K Shift+T"),
                true,
                None,
            ),
        ],
        preview: CommandPreviewProjection {
            title: "Run Project".to_string(),
            body: vec![
                "Target: ExcelDesktop".to_string(),
                "Runs the current OxVba project through the configured host.".to_string(),
            ],
        },
        footer_hints: vec![
            action("Run selected", "Enter", "command.execute_selected"),
            action("Alternate", "Ctrl+Enter", "command.execute_alternate"),
            action("Close", "Esc", "overlay.close"),
        ],
    }));
    projection.key_rail = key_rail(&[
        action("Type to filter", "text", "command.filter.update"),
        action("Run selected", "Enter", "command.execute_selected"),
        action("Preview", "Tab", "command.preview.focus"),
        action("Close", "Esc", "overlay.close"),
    ]);
    projection
}

fn run_lane() -> FireHorseProjection {
    let layout = LayoutPosture::RunLane;
    let events = vec![
        run_event(
            "ExcelDesktop",
            RunStepKind::Prepare,
            RunStepStatus::Complete,
            10,
        ),
        run_event(
            "ExcelDesktop",
            RunStepKind::Analyze,
            RunStepStatus::Complete,
            180,
        ),
        run_event(
            "ExcelDesktop",
            RunStepKind::Build,
            RunStepStatus::Active,
            430,
        ),
        run_event(
            "ExcelDesktop",
            RunStepKind::Execute,
            RunStepStatus::Pending,
            0,
        ),
        run_event(
            "ExcelDesktop",
            RunStepKind::Result,
            RunStepStatus::Pending,
            0,
        ),
    ];

    FireHorseProjection {
        scenario_id: "firehorse-run-lane-standard",
        expected_layout: layout,
        identity: identity(layout, "NorthwindPricing", Some("ExcelDesktop"), None),
        project_spine: Some(project_spine()),
        code_canvas: CodeCanvasProjection {
            document_label: "PriceFor.bas".to_string(),
            language: "VBA",
            lines: pricing_source_lines(),
            lens: None,
            execution_line: None,
            selection: None,
        },
        context_dock: Some(ContextDockProjection {
            title: "Run Status".to_string(),
            cards: vec![ContextCardProjection::RunStatus(RunStatusProjection {
                target_id: "ExcelDesktop".to_string(),
                active_step: RunStepKind::Build,
                status: RunStepStatus::Active,
                message: "Building host package".to_string(),
            })],
        }),
        activity_deck: ActivityDeckProjection {
            posture: DeckPosture::Expanded,
            active: ActivityKind::RunTimeline,
            tabs: tabs(&[
                (ActivityKind::RunTimeline, "Run Timeline", Some(5)),
                (ActivityKind::Output, "Output", Some(3)),
                (ActivityKind::Immediate, "Immediate", None),
            ]),
            rows: events
                .iter()
                .cloned()
                .map(ActivityRowProjection::RunEvent)
                .collect(),
        },
        key_rail: key_rail(&[
            action("Stop Run", "F8", "run.stop"),
            action("Immediate", "Ctrl+G", "immediate.focus"),
            action("Return", "Esc", "scene.return_edit"),
        ]),
        overlay: None,
        theme: ThemeProjection::GraphiteEmber,
        terminal_fit: None,
        layout,
        seams: SeamFixtureSet {
            run_events: events,
            ..SeamFixtureSet::empty()
        },
    }
}

fn debug_cockpit() -> FireHorseProjection {
    let layout = LayoutPosture::DebugCockpit;
    let frames = vec![
        StackFrameProjection {
            frame_id: "frame-0".to_string(),
            procedure: "PriceFor".to_string(),
            document_id: "doc://NorthwindPricing/PriceFor.bas".to_string(),
            line: 8,
            source: debug_source("call_stack"),
        },
        StackFrameProjection {
            frame_id: "frame-1".to_string(),
            procedure: "QuoteSelectedOrder".to_string(),
            document_id: "doc://NorthwindPricing/Orders.bas".to_string(),
            line: 22,
            source: debug_source("call_stack"),
        },
    ];
    let locals = vec![LocalValueProjection {
        name: "productId".to_string(),
        type_label: "Long".to_string(),
        value: "42".to_string(),
        source: debug_source("locals"),
    }];
    let watches = vec![WatchProjection {
        expression: "PriceFor(productId)".to_string(),
        type_label: "Currency".to_string(),
        value: "12.40".to_string(),
        source: debug_source("watches"),
    }];

    FireHorseProjection {
        scenario_id: "firehorse-debug-cockpit-standard",
        expected_layout: layout,
        identity: identity(layout, "NorthwindPricing", Some("ExcelDesktop"), None),
        project_spine: Some(project_spine()),
        code_canvas: CodeCanvasProjection {
            document_label: "PriceFor.bas".to_string(),
            language: "VBA",
            lines: pricing_source_lines(),
            lens: None,
            execution_line: Some(8),
            selection: None,
        },
        context_dock: Some(ContextDockProjection {
            title: "Debug Cockpit".to_string(),
            cards: vec![
                ContextCardProjection::CallStack(frames.clone()),
                ContextCardProjection::Locals(locals.clone()),
                ContextCardProjection::Watches(watches.clone()),
            ],
        }),
        activity_deck: ActivityDeckProjection {
            posture: DeckPosture::Expanded,
            active: ActivityKind::WatchTrace,
            tabs: tabs(&[
                (ActivityKind::WatchTrace, "Watch/Trace", Some(1)),
                (ActivityKind::Immediate, "Immediate", Some(2)),
                (ActivityKind::Output, "Output", Some(5)),
            ]),
            rows: vec![
                ActivityRowProjection::StackFrame(frames[0].clone()),
                ActivityRowProjection::Local(locals[0].clone()),
                ActivityRowProjection::Watch(watches[0].clone()),
            ],
        },
        key_rail: key_rail(&[
            action("Continue", "F5", "debug.continue"),
            action("Step", "F8", "debug.step"),
            action("Step Out", "Shift+F8", "debug.step_out"),
            action("Pin Watch", "Ctrl+W", "watch.pin"),
            action("Return", "Esc", "scene.return_edit"),
        ]),
        overlay: None,
        theme: ThemeProjection::GraphiteEmber,
        terminal_fit: None,
        layout,
        seams: SeamFixtureSet {
            debug_frames: frames,
            locals,
            watches,
            ..SeamFixtureSet::empty()
        },
    }
}

fn console_fit() -> FireHorseProjection {
    let layout = LayoutPosture::ConsoleFit;
    let fit = TerminalFitProjection {
        summary: "Truecolor pass; glyph fallback warning; input latency pass".to_string(),
        rows: vec![
            TerminalFitRowProjection {
                signal: "truecolor",
                result: FitResult::Pass,
                detail: "24-bit color supported".to_string(),
                recommendation: "Use Graphite Ember or Paper Ember themes.".to_string(),
            },
            TerminalFitRowProjection {
                signal: "box-glyphs",
                result: FitResult::Warn,
                detail: "Heavy separators may degrade in this host".to_string(),
                recommendation: "Prefer ASCII rail fallback for dense scenes.".to_string(),
            },
            TerminalFitRowProjection {
                signal: "alternate-screen",
                result: FitResult::Pass,
                detail: "ConPTY alternate screen available".to_string(),
                recommendation: "Safe for WTD capture.".to_string(),
            },
        ],
    };

    FireHorseProjection {
        scenario_id: "firehorse-console-fit-light",
        expected_layout: layout,
        identity: identity(layout, "Console Fit", None, None),
        project_spine: None,
        code_canvas: CodeCanvasProjection {
            document_label: "ConsoleFit".to_string(),
            language: "terminal-capability",
            lines: vec![],
            lens: None,
            execution_line: None,
            selection: None,
        },
        context_dock: Some(ContextDockProjection {
            title: "Capability Detail".to_string(),
            cards: vec![ContextCardProjection::TerminalFit(fit.clone())],
        }),
        activity_deck: ActivityDeckProjection {
            posture: DeckPosture::Expanded,
            active: ActivityKind::Output,
            tabs: tabs(&[(ActivityKind::Output, "Signals", Some(3))]),
            rows: fit
                .rows
                .iter()
                .map(|row| ActivityRowProjection::Text {
                    source: "TerminalCapabilityProbe",
                    text: format!("{}: {:?} - {}", row.signal, row.result, row.recommendation),
                })
                .collect(),
        },
        key_rail: key_rail(&[
            action("Open Report", "Enter", "console_fit.report.open"),
            action("Rerun Checks", "Enter", "console_fit.rerun"),
            action("Return", "Esc", "scene.return_edit"),
        ]),
        overlay: None,
        theme: ThemeProjection::PaperEmber,
        terminal_fit: Some(fit),
        layout,
        seams: SeamFixtureSet::empty(),
    }
}

fn focus_compact() -> FireHorseProjection {
    let layout = LayoutPosture::CompactFocus;
    FireHorseProjection {
        scenario_id: "firehorse-focus-compact",
        expected_layout: layout,
        identity: identity(
            layout,
            "NorthwindPricing",
            Some("ExcelDesktop"),
            Some(CursorProjection { line: 8, column: 9 }),
        ),
        project_spine: None,
        code_canvas: CodeCanvasProjection {
            document_label: "PriceFor.bas".to_string(),
            language: "VBA",
            lines: pricing_source_lines(),
            lens: Some(SourceLensProjection {
                anchor: range(8, 5, 8, 16),
                title: "Compact source lens".to_string(),
                body: vec!["Project, Context, and Activity are temporary docks.".to_string()],
                actions: vec![
                    action("Project", "Alt+1", "focus.project"),
                    action("Context", "Alt+3", "focus.context"),
                    action("Activity", "Alt+4", "focus.activity"),
                ],
                source: SeamSourceProjection {
                    provider: "LayoutPolicy",
                    query: "compact-focus-affordances",
                },
            }),
            execution_line: None,
            selection: Some(range(8, 5, 8, 16)),
        },
        context_dock: None,
        activity_deck: ActivityDeckProjection {
            posture: DeckPosture::Compact,
            active: ActivityKind::Problems,
            tabs: tabs(&[(ActivityKind::Problems, "Problems", Some(1))]),
            rows: vec![ActivityRowProjection::Diagnostic(pricing_diagnostic())],
        },
        key_rail: key_rail(&[
            action("Project", "Alt+1", "focus.project"),
            action("Code", "Alt+2", "focus.code"),
            action("Context", "Alt+3", "focus.context"),
            action("Activity", "Alt+4", "focus.activity"),
            action("Command", "F6", "command.lens.open"),
        ]),
        overlay: None,
        theme: ThemeProjection::GraphiteEmber,
        terminal_fit: None,
        layout,
        seams: SeamFixtureSet {
            diagnostics: vec![pricing_diagnostic()],
            symbols: vec![price_for_symbol()],
            ..SeamFixtureSet::empty()
        },
    }
}

fn identity(
    scene: LayoutPosture,
    workspace_label: &str,
    target: Option<&str>,
    cursor: Option<CursorProjection>,
) -> IdentityRailProjection {
    IdentityRailProjection {
        product: "OxIde",
        workspace_label: workspace_label.to_string(),
        scene,
        target: target.map(str::to_string),
        health: vec![StateBadgeProjection {
            label: "Ready".to_string(),
            tone: BadgeTone::Success,
        }],
        cursor,
    }
}

fn project_spine() -> ProjectSpineProjection {
    ProjectSpineProjection {
        posture: SpinePosture::Full,
        rows: vec![
            project_row("NorthwindPricing", ProjectItemKind::Project, 0, true),
            project_row("PriceFor.bas", ProjectItemKind::Module, 1, true),
            project_row("Orders.bas", ProjectItemKind::Module, 1, false),
            project_row("CustomerView.frm", ProjectItemKind::Form, 1, false),
            project_row("ExcelDesktop", ProjectItemKind::Target, 1, false),
            project_row("VBA", ProjectItemKind::Reference, 1, false),
        ],
    }
}

fn project_row(
    label: &str,
    kind: ProjectItemKind,
    depth: u8,
    active: bool,
) -> ProjectSpineRowProjection {
    ProjectSpineRowProjection {
        label: label.to_string(),
        kind,
        depth,
        active,
        dirty: label == "PriceFor.bas",
        badges: if active {
            vec![StateBadgeProjection {
                label: "active".to_string(),
                tone: BadgeTone::Info,
            }]
        } else {
            vec![]
        },
        seam_ref: Some(ProjectSeamRef {
            project_id: "NorthwindPricing".to_string(),
            item_id: label.to_string(),
        }),
    }
}

fn welcome_canvas() -> CodeCanvasProjection {
    CodeCanvasProjection {
        document_label: "Launchpad".to_string(),
        language: "workspace-launcher",
        lines: vec![
            source_line(1, "Open or create an OxVba project.", vec![]),
            source_line(2, "Recent workspaces are session-store fixtures.", vec![]),
        ],
        lens: None,
        execution_line: None,
        selection: None,
    }
}

fn pricing_source_lines() -> Vec<SourceLineProjection> {
    vec![
        source_line(1, "Option Explicit", vec![]),
        source_line(
            3,
            "Public Function PriceFor(productId As Long) As Currency",
            vec![],
        ),
        source_line(4, "    Dim basePrice As Currency", vec![]),
        source_line(5, "    basePrice = LookupPrice(productId)", vec![]),
        source_line(6, "    PriceFor = ApplyRegionalDiscount(basePrice)", vec![]),
        source_line(
            8,
            "    Debug.Print FormatCurrency(PriceFor)",
            vec![GutterMarkerProjection::Diagnostic],
        ),
        source_line(9, "End Function", vec![]),
    ]
}

fn source_line(
    number: u32,
    text: &str,
    markers: Vec<GutterMarkerProjection>,
) -> SourceLineProjection {
    SourceLineProjection {
        number,
        text: text.to_string(),
        markers,
        semantic_spans: vec![],
    }
}

fn pricing_diagnostic() -> MockDiagnosticProjection {
    MockDiagnosticProjection {
        document_id: "doc://NorthwindPricing/PriceFor.bas".to_string(),
        range: range(8, 34, 8, 42),
        severity: DiagnosticSeverity::Warning,
        code: "OXVBA-W060".to_string(),
        message: "Argument to FormatCurrency is inferred as Variant; inspect return type."
            .to_string(),
        provenance: DiagnosticProvenanceProjection {
            provider: "HostWorkspaceSession::diagnostics",
            project_id: "NorthwindPricing".to_string(),
        },
    }
}

fn price_for_symbol() -> MockSymbolProjection {
    MockSymbolProjection {
        document_id: "doc://NorthwindPricing/PriceFor.bas".to_string(),
        range: range(6, 5, 6, 13),
        kind: SymbolKind::Function,
        name: "PriceFor".to_string(),
        detail: "Public Function PriceFor(productId As Long) As Currency".to_string(),
        provenance: SymbolProvenanceProjection {
            provider: "HostWorkspaceSession::hover",
            document_id: "doc://NorthwindPricing/PriceFor.bas".to_string(),
        },
    }
}

fn run_event(
    target_id: &str,
    step: RunStepKind,
    status: RunStepStatus,
    emitted_at_ms: u64,
) -> MockRunEventProjection {
    MockRunEventProjection {
        target_id: target_id.to_string(),
        step,
        status,
        message: format!("{:?} is {:?}", step, status),
        emitted_at_ms,
    }
}

fn debug_source(query: &'static str) -> SeamSourceProjection {
    SeamSourceProjection {
        provider: "OxVba debug contract (W080 audit)",
        query,
    }
}

fn range(
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
) -> SourceRangeProjection {
    SourceRangeProjection {
        start: SourcePositionProjection {
            line: start_line,
            column: start_column,
        },
        end: SourcePositionProjection {
            line: end_line,
            column: end_column,
        },
    }
}

fn tabs(values: &[(ActivityKind, &str, Option<u32>)]) -> Vec<ActivityTabProjection> {
    values
        .iter()
        .map(|(kind, label, count)| ActivityTabProjection {
            kind: *kind,
            label: (*label).to_string(),
            count: *count,
        })
        .collect()
}

fn key_rail(hints: &[ActionHintProjection]) -> KeyRailProjection {
    KeyRailProjection {
        hints: hints.to_vec(),
        no_wrap: true,
    }
}

fn action(label: &str, binding: &str, action_id: &'static str) -> ActionHintProjection {
    ActionHintProjection {
        label: label.to_string(),
        binding: KeyBindingProjection {
            label: binding.to_string(),
        },
        action_id,
        enabled: true,
        disabled_reason: None,
        display_only_reason: None,
    }
}

fn command_row(
    action_id: &'static str,
    label: &str,
    binding: Option<&str>,
    enabled: bool,
    disabled_reason: Option<&str>,
) -> CommandRowProjection {
    CommandRowProjection {
        action_id,
        label: label.to_string(),
        binding: binding.map(|binding| KeyBindingProjection {
            label: binding.to_string(),
        }),
        enabled,
        disabled_reason: disabled_reason.map(str::to_string),
        preview: CommandPreviewProjection {
            title: label.to_string(),
            body: vec![format!("Action id: {action_id}")],
        },
    }
}

impl SeamFixtureSet {
    fn empty() -> Self {
        Self {
            diagnostics: vec![],
            symbols: vec![],
            run_events: vec![],
            debug_frames: vec![],
            locals: vec![],
            watches: vec![],
        }
    }
}
