use dialoguer::{Confirm, Select, theme::ColorfulTheme};

pub fn prompt_compression(theme: &ColorfulTheme) -> anyhow::Result<(Option<u32>, u8)> {
    let do_compress = Confirm::with_theme(theme)
        .with_prompt("🗜  Make images smaller?")
        .default(false)
        .interact()?;

    if !do_compress {
        return Ok((None, 90));
    }

    let presets = &[
        "Small – chat / thumbnails",
        "Medium – AI / web (recommended)",
        "Large – keep more detail",
        "No resize – only strip metadata",
        "Advanced – tweak size and JPEG quality",
    ];

    let choice = Select::with_theme(theme)
        .with_prompt("   How will you use these images?")
        .items(presets)
        .default(1)
        .interact()?;

    Ok(match choice {
        0 => {
            println!("     → Small (1024 px, quality 85).");
            (Some(1024), 85)
        }
        1 => {
            println!("     → Medium (2048 px, quality 90).");
            (Some(2048), 90)
        }
        2 => {
            println!("     → Large (4096 px, quality 95).");
            (Some(4096), 95)
        }
        3 => {
            println!("     → No resize, strip metadata only.");
            (None, 90)
        }
        4 => ask_advanced(theme)?,
        _ => (Some(2048), 90),
    })
}

fn ask_advanced(theme: &ColorfulTheme) -> anyhow::Result<(Option<u32>, u8)> {
    let size_options = &[
        "1024 px – small",
        "2048 px – medium",
        "4096 px – large",
        "No resize",
    ];
    let shrink = match Select::with_theme(theme)
        .with_prompt("   Max image dimension")
        .items(size_options)
        .default(1)
        .interact()?
    {
        0 => Some(1024),
        1 => Some(2048),
        2 => Some(4096),
        _ => None,
    };

    println!(
        "     Higher quality = better detail, bigger files. Lower = smaller, may show artifacts."
    );

    let quality_options = &[
        "Smaller files (75)  – may look a bit rough",
        "Balanced (90)       – looks like original  ← recommended",
        "Max quality (100)   – largest files",
    ];
    let quality = match Select::with_theme(theme)
        .with_prompt("   Quality vs file size?")
        .items(quality_options)
        .default(1)
        .interact()?
    {
        0 => 75,
        1 => 90,
        2 => 100,
        _ => 90,
    };

    Ok((shrink, quality))
}
