use plotters::prelude::*;

type T = f32;

const PLOT_RES: (u32, u32) = (1024, 760);
const PLOT_CAPTION_FONT: (&str, u32) = ("sans", 20);
const PLOT_MARGIN: u32 = 5;
const PLOT_LABEL_AREA_SIZE: u32 = 30;

#[allow(unexpected_cfgs)]
pub fn plot_curves_anim<const N: usize, const M: usize>(
    plot_title: &str, plot_path: &str,
    x: [[&[T]; N]; M],
    y: [[&[T]; N]; M],
    time: T
) -> Result<(), Box<dyn std::error::Error>>
{
    #[cfg(not(tarpaulin))]
    {
        let x_min = x.into_iter().flatten().flatten().copied().reduce(T::min).unwrap();
        let x_max = x.into_iter().flatten().flatten().copied().reduce(T::max).unwrap();
        
        let y_min = y.into_iter().flatten().flatten().copied().reduce(T::min).unwrap();
        let y_max = y.into_iter().flatten().flatten().copied().reduce(T::max).unwrap();
        
        let show = (1000.0*time/M as T) as u32;

        let area = BitMapBackend::gif(
                plot_path,
                PLOT_RES,
                show
            ).unwrap()
            .into_drawing_area();
        
        for (x, y) in x.into_iter().zip(y)
        {
            area.fill(&WHITE)?;
            
            let mut chart = ChartBuilder::on(&area)
                .caption(plot_title, PLOT_CAPTION_FONT.into_font())
                .margin(PLOT_MARGIN)
                .x_label_area_size(PLOT_LABEL_AREA_SIZE)
                .y_label_area_size(PLOT_LABEL_AREA_SIZE)
                .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
            
            chart.configure_mesh()
                .set_all_tick_mark_size(0.1)
                .draw()?;
            
            for (i, (x, y)) in x.into_iter().zip(y).enumerate()
            {
                let mut j = i;
                if j == 2
                {
                    j += 1
                }
                let color = Palette99::pick(j);
                chart.draw_series(LineSeries::new(
                        x.into_iter().copied().zip(y.into_iter().copied()),
                        &color
                    ))?
                    .label(format!("{}", i))
                    .legend(move |(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], color.mix(0.5).filled()));
            }
            
            chart.configure_series_labels()
                .border_style(&BLACK)
                .draw()?;
            
            area.present().expect("Unable to write result to file");   
        }
    }

    Ok(())
}