# fonttest

*Things are changing fast. This is the situation on 2020-01-07.*

Font-size in html and css is just crazy.  
If the android chrome has accessibility set to bigger fonts it destroys the layout of the game.  
Yes, the problem is that HTML was made for magazines and articles.  
They don't care about font size.  
Let the user choose his own font size, they say.  
But for games the size on the screen is everything.  
If there is just a few pixels difference, the text line will wrap and will destroy the layout.  
There is a way out using Canvas or SVG, but I want to use HTML and css.  

## not proportional

I was surprised to fond out that chrome makes not proportional changes to the fonts.  
The big fonts are usually unchanged. Smaller the font, the bigger is the change they make.  
It makes sense, but is so much more difficult to "correct".  

## CPL - characters per line

Mostly I don't really care about the size of the characters.
I care more about how many characters will stay in one line without word-wrap.  
There is a unit of measure for that. It is CPL - characters per line.  
I will try to reinvent this new unit.  
In the start of the wasm code I will try to find what font-size px produces what cpl.  
I will do this for many different font-size px because they don't produce proportional font sizes.  
Then somehow I will apply this to the css styles.  

## css custom properties (variables)

It is possible to add variables to css.  
If I change this variables, the css could be easy to read and interpret.  
:root {
    --somevar: black;
}
In javascript/wasm or easm I could change them:
document.documentElement.style.setProperty('--somevar', 'green');

## text.offset_width

The numbers I get for text width are not the same as I see the font size is.  
If I cannot get the last numbers, then I cannot change the size.  
It looks that the numbers are calculated before the browser increases the font-size.  
It means I cannot know how it increases the size from javascript.  

## SVG

It looks that I must use SVG text and fonts. Sadly complicated.  

## build and run

Type and run this in the project folder  
`cargo make dev` or `cargo make release`  
To install cargo-make  
`cargo install --force cargo-make`  
