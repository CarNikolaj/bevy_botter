use bevy::prelude::*;
use iyes_perf_ui::prelude::*;




pub fn performance(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            //values_col_width: 32.0,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
        PerfUiWidgetBar::new(PerfUiEntryFrameTime::default()),
        PerfUiWidgetBar::new(PerfUiEntryFrameTimeWorst::default()),
        PerfUiWidgetBar::new(PerfUiEntryRenderCpuTime::default()),
        PerfUiWidgetBar::new(PerfUiEntryRenderGpuTime::default()),
        PerfUiWidgetBar::new(PerfUiEntryCpuUsage::default()),
        PerfUiWidgetBar::new(PerfUiEntryMemUsage::default()),
        PerfUiEntryWindowResolution::default(),


    ));
}
