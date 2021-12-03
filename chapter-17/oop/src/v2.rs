pub struct Post {
  content: String,
}

pub struct DraftPost {
  content: String,
}

pub struct PendingTwoReviewsPost {
  content: String,
}

pub struct PendingOneReviewPost {
  content: String,
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingTwoReviewsPost {
    PendingTwoReviewsPost {
      content: self.content,
    }
  }
}

impl PendingTwoReviewsPost {
  pub fn approve(self) -> PendingOneReviewPost {
    PendingOneReviewPost {
      content: self.content,
    }
  }
}

impl PendingOneReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}
