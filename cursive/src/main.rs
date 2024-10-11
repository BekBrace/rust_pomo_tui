use cursive::Cursive;

use cursive::views::TextView;

use cursive::views::Dialog;
fn main() {
    let mut siv = cursive::default();
    
    siv.add_global_callback('q', |s| s.quit());
    
    // siv.add_layer(TextView::new("Hello, World! Press <q> to quit!"));
    
    // siv.add_layer(Dialog::around(TextView::new("Question1 :")));
    
    
    // siv.add_layer(Dialog::text("Do you want to format Disk Drive?").title("Computer Manager"));

//     siv.add_layer(Dialog::text("Are you done ?")
//     .title("Personal Organizer")    
//     .button("Yes!", |s| s.quit())
//     .button("No!", |s| s.quit())
//     .button("Not Sure!", |s| s.quit())
// );

siv.add_layer(
    Dialog::text("Have you used ICQ before?")
    .title("retro test")
    .button("Yes!", yes)
    .button("No!", no)
);    
    siv.run();
}


fn yes(s: &mut Cursive){
    s.pop_layer();
    s.add_layer(TextView::new("Sweet! You're a retro guy!"))
}
fn no(s: &mut Cursive){
    s.pop_layer();
    s.add_layer(TextView::new("Sorry! You're too young!"))
}