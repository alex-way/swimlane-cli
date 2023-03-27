use std::fmt::Display;

use colored::Color;
use colored::Colorize;
use swimlane_migrator::equality::{Difference, LooksLike};
use swimlane_migrator::MigrationPlan;

// todo: Move this to the Display trait for MigrationPlan?
pub fn dry_run_resource_migrate<T: LooksLike + Display>(plans: Vec<MigrationPlan<T>>) {
    {
        if plans.is_empty() {
            println!(
                "{}",
                "No changed detected. Everything is up to date.".green()
            );
            return;
        }

        plans.iter().for_each(|plan| {
            if let MigrationPlan::Delete {
                destination_resource,
            } = plan
            {
                println!(
                    "{}",
                    format!("{} will be deleted", destination_resource).red()
                )
            }
        });

        plans.iter().for_each(|plan| {
            if let MigrationPlan::Create { source_resource } = plan {
                println!("{}", format!("{} will be created", source_resource).green())
            }
        });

        plans.iter().for_each(|plan| {
            if let MigrationPlan::Update {
                source_resource,
                destination_resource,
            } = plan
            {
                println!(
                    "{}",
                    format!("{} will be updated", source_resource).yellow()
                );
                for difference in source_resource.differences(destination_resource) {
                    let color = match difference {
                        Difference::UpdatingField { .. } => Color::Yellow,
                        Difference::AddingItem { .. } => Color::Green,
                        Difference::RemovingItem { .. } => Color::Red,
                    };
                    println!("\t{}", format!("{}", difference).color(color));
                }
            }
        });
    }
}
