use crate::vm_maps::vm_maps::{VMMap, VMMaps};

pub struct PageMapStats;

impl PageMapStats {
    fn total_pages(pm: &VMMap) -> usize {
        pm.pagemap.len()
    }

    fn present_pages(pm: &VMMap) -> usize {
        pm.pagemap.iter().filter(|page| page.present).count()
    }

    fn dead_pages(pm: &VMMap) -> usize {
        Self::total_pages(pm) - Self::present_pages(pm)
    }

    fn ram_percentage(pm: &VMMap) -> f64 {
        let present_pages = Self::present_pages(pm);
        if present_pages > 0 {
            (present_pages as f64 / Self::total_pages(pm) as f64) * 100.0
        } else {
            0.0
        }
    }

    fn pm_stats_description(pm: &VMMap) -> String {
        format!(
            "Pathname {} has mapped {} page(s) in total.
            Out of them, present in RAM currently are {}, not present in RAM are {}
            Percentage of present in RAM pages: {}%",
            pm.maps.pathname,
            Self::total_pages(pm),
            Self::present_pages(pm),
            Self::dead_pages(pm),
            Self::ram_percentage(pm)
        )
    }

    pub fn stats_description(p: &VMMaps) -> String {
        let result: Vec<String> = p
            .snapshot()
            .iter()
            .map(Self::pm_stats_description)
            .collect();
        result.join("\n")
    }
}
