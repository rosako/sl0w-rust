use std::io::{self, stdout};

use shmemlib::{Segment, shm_get_struct, test_dataset, shm_get_segments};

use ratatui::prelude::*;

use ratatui::{
    symbols::border,
    layout::{
        Constraint,
        Layout, Rect,
        Constraint::{Length, Max, Min, Percentage, Ratio},
    },
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget, Borders,
        List, ListItem,
    },
    Frame, Terminal,
};


static mut current_line: i32 = 0;
static mut current_num_lines: i32 = 0;



struct ShmemWidget {
    key: String,
    shmid: String,
    owner: String,
    perm: String,
    bytes: String,
    nattch: String
}

impl ShmemWidget{

    /*
    pub fn new(key: String) -> Self{
        Self { key }
    }
    */

}


impl Widget for ShmemWidget {
    /*
    fn render(self, area: Rect, buf: &mut Buffer) {
        let arrow = Span::raw("> ");
        let key = Span::styled(self.key, Modifier::BOLD);
        let line = Line::from(vec![arrow, key]);
        line.render(area, buf);
    }
    */

    fn render(self, area: Rect, buf: &mut Buffer){

        
        let title = Title::from( "Shmem Inspector".bold());


        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
        ])]);



        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);

    }
}



fn main() -> io::Result<()> {

    //let segment = shm_get_struct(32778);


    //input is not directly written to the terminal   
    enable_raw_mode()?;

    // create a separate screen buffer, so the app can take over the entire terminal => dedicated
    // display to the app
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    
    // hold the boolean that determines whether it should quit
    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        // decide from the handle_events() what should be next step
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;


    Ok(())


}


// function that is responsible for analysing the context and decide if we quit, based on key
// presses

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()?{

            // quit
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
            
            // move down
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('j') {
                unsafe {
                    //current_line = (current_line+1)%current_num_lines;
                    if(current_line < current_num_lines-1){
                        current_line = current_line + 1;
                    }
                }
            }

            // move up
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('k') {
                unsafe {
                    if(current_line > 0){
                        current_line = (current_line-1)%current_num_lines;
                    }
                }
            }


            // move top
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('t') {
                unsafe {
                    current_line = 0;
                }
            }

            // move bottom
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('b') {
                unsafe {
                    current_line = current_num_lines-1;
                }
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    //ShmemWidget


    let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                         Constraint::Length(3),
                         Constraint::Min(1),
                         Constraint::Length(3),
            ])
            .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let curr_line;

    unsafe{
        curr_line = current_line;
    }

    let title = Paragraph::new(Text::styled(
            
            "|----- Sl0w Shmemspector v001 | Roland Sako -----|",
            Style::default().fg(Color::Green),
            ))
        .block(title_block)
        .alignment(Alignment::Center);

    
    frame.render_widget(title, chunks[0]);


    // -- second part
    let mut list_items = Vec::<ListItem>::new();


    list_items.push(ListItem::new(Line::from(Span::styled(format!("key\t\t\tshmid\t\towner\t\t\tperms\t\tbytes\t\tnattch"), Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC).fg(Color::Yellow).bg(Color::White)))));

    //let segments = test_dataset();

    let segments = shm_get_segments(50000);
    unsafe{
        current_num_lines = segments.len() as i32;
    }


    for i in 0..segments.len() {
        let line = segments[i].to_line();
        let line_ = &segments[i];
        let line_color:Color;
        if i == curr_line as usize{
            line_color = Color::Yellow;
        }else{
            line_color = Color::Green; 
        }
        list_items.push(ListItem::new(Line::from(Span::styled(format!("{}\t\t{}\t\t{}\t\t{}\t\t{}\t\t{}", line_.key, line_.shmid, line_.owner, line_.perms, line_.segsz, line_.nattch), Style::default().fg(line_color)))));
    }

    
    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);

    // -- third part
    let footer = Paragraph::new("Press <q> to quit")
        .block(Block::bordered().title("Help"))
        .style(Style::default().green())//new().white().on_black())
        .alignment(Alignment::Center);

    frame.render_widget(footer, chunks[2]);


    /*
    frame.render_widget(
        ShmemWidget::new(String::from("0x1337")),
        frame.size(),

    );
    */

    /*
    frame.render_widget(
        Paragraph::new("Hello World!").block(Block::bordered().title("ShmemInspector")),
        frame.size(),
    );
    */

}
