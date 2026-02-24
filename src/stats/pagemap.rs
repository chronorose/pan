use crate::vm_maps::pages_snapshot::{PageMap, PageMapSnapshot};

pub struct PageMapStats;

impl PageMapStats {
    fn total_pages(pm: &PageMap) -> usize {
        pm.pages.len()
    }

    fn present_pages(pm: &PageMap) -> usize {
        pm.pages.iter().filter(|page| page.present).count()
    }

    fn dead_pages(pm: &PageMap) -> usize {
        Self::total_pages(pm) - Self::present_pages(pm)
    }

    fn ram_percentage(pm: &PageMap) -> f64 {
        let present_pages = Self::present_pages(pm);
        if present_pages > 0 {
            (present_pages as f64 / Self::total_pages(pm) as f64) * 100.0
        } else {
            0.0
        }
    }

    fn pm_stats_description(pm: &PageMap) -> String {
        format!(
            "Pathname {} has mapped {} page(s) in total.
            Out of them, present in RAM currently are {}, not present in RAM are {}
            Percentage of present in RAM pages: {}%",
            pm.map.pathname,
            Self::total_pages(pm),
            Self::present_pages(pm),
            Self::dead_pages(pm),
            Self::ram_percentage(pm)
        )
    }

    pub fn stats_description(p: &PageMapSnapshot) -> String {
        let result: Vec<String> = p.snapshot.iter().map(Self::pm_stats_description).collect();
        result.join("\n")
    }
}
