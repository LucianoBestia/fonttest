# fonttest

*Things are changing fast. This is the situation on 2020-01-07.*

Font-size in html and css is just crazy.  
If the android chrome has accessibility set to bigger fonts it destroys the layout of the game.  
Yes, the problem is that HTML was made for magazines and articles.  
They don't care about font size.  
Let the user choose his own font size.  
But for games the size on the screen is everything.  
If there is just a few pixels difference, the text line will wrap and will destroy the layout.  
There is a way out using Canvas or SVG, but I want to use HTML and css.  

I will try different things.

## build and run

Type and run this in the project folder  
`cargo make dev` or `cargo make release`  
To install cargo-make  
`cargo install --force cargo-make`  
