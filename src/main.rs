use cursive;

fn main() {
  let mut siv = cursive::default();

  siv.add_global_callback('q', |s| s.quit());

  first_slide(&mut siv);

  siv.run();
}


fn first_slide(s: &mut cursive::Cursive) {
  s.add_layer(cursive::views::Dialog::text("By William Lane").title("Depression").button("Next", |s| second_slide(s)));
}

fn second_slide(s: &mut cursive::Cursive) {
  s.pop_layer();
  s.add_layer(
    cursive::views::Dialog::text(
      "Depression is a mood disorder that causes distressing symptoms that\n
      changes how you feel, think, and handle daily activities."
      ).title("What is Depression?")
    .button("Next slide", |s| third_slide(s))
    );
}

fn third_slide(s: &mut cursive::Cursive) {
  s.pop_layer();
  s.add_layer(
    cursive::views::Dialog::text(
      "Globally, depression affects more than 267,000,000 people\nwhich is almost the population of the united states."
      ).title("How Many People Suffer From Depression?")
    .button("Next slide", |s| fourth_slide(s))
    );
}

fn fourth_slide(s: &mut cursive::Cursive) {
  s.pop_layer();
  s.add_layer(
    cursive::views::Dialog::text(
      "Good news, about 60 to 80 percent of people with\ndepression can be treated successfully.\nOne method of treatment is to focus on improving\nrelationships with people to reduce feelings of despair."
      ).title("How Can Depression Be Treated?")
    .button("Next slide", |s| sources(s))
    );
}

fn sources(s: &mut cursive::Cursive) {
  s.pop_layer();
  s.add_layer(
    cursive::views::Dialog::text("Facts about DEPRESSION, www.idph.state.il.us/about/womenshealth/factsheets/dep.htm.\nDepression.” World Health Organization, World Health Organization, www.who.int/news-room/fact-sheets/detail/depression.\nDepression Basics.” National Institute of Mental Health, U.S. Department of Health and Human Services, www.nimh.nih.gov/health/publications/depression/index.shtml."
      ).title("Sources")
    .button("Thank you for listening", |s| s.quit())
    );
}
