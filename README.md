Find a Rust UI (CLI) library and integrate the following functionalities:
  - Add method to draw a main screen (for main menu and board)
  - Add method to draw a overlay/popup screen (for pause and victory screen)
  - Add method to save and load screen as json,binary,etc
  - Add option to have colored output
  - Add option to have header and footer area
  - Try to abstract these methods as much as possible for convenience
  - Possibly add option to interact with the mouse
  - Investiage Unicode support and in best case pixel drawings or small images

Main Menu:
  - Add option to input number of players
  - Add option to start a game via button or keyboard input
  - Add option to load a game via button or keyboard input
  - Add option to close the game

Pause Menu:
  - Add option to resume the game
  - Add option to save the current gmae (maybe even CTRL + S)
  - Add option to load other game
  - Add option to go back to main menu
  - Add option to exit the game

Game loop:
  - Create game phase
  - Initialization phase
  - Game phase
  - End phase

Create Game Phase:
  - User provides an input of how many players will play
  - Colors are assigned to the player
  - Game proceeds to Initialization Phase

Initialization Phase:
  - Initialize board with each player in it's starting position:
    - Board contains total of 40 playing fields (field) that are connected in circle
    - Board contains an area (home positions) with 4 players pieces (pawns) stored in each corner
    - Every player has his starting position (start) which is located on every 10th field
    - Every player has safe spot (safehouses) which is located at the end of the field (full circle)
  - Initial state is added to memory/storage
  - Each player throws a six-sided dice three times:
    - Throwing can be implemented with keyboard or mouse
  - First player to get a 6 starts off the game by moving its first piece to the "base" position
  - Game proceeds to Game Phase

Game Phase:
  - Current player is loaded into memory
  - Player throws a six-sided dice:
    - If the dice rolled number different than six allow player to move to a valid piece
    - If the dice rolled number six allow player to either:
      - Move his own piece from the home positions, if there are any left, to the base position only if it's not occupied by his own piece
      - Move 6 positions
  - Mark players available spots with color and allow players to choose
  - Available spots logic:
    - Forbid him to:
      - Move backwards
      - Move a pawn to a location where he already has a pawn
      - Skip his own pawns in the safehouse
    - Place marker on the field, for every players pawn on the field, that is ahead (clockwise) of the current field by the number the player threw if:
      - The calculated field does not:
        - Make a full circle (is not out of bounds)
        - Field does not have a players piece
        - Skip over any pawns in safehouse fields
  - If the player gathered all 4 of his pawns in their designeted safehouses:
    - Go to End Phase
  - Else
    - If player has thrown six:
      - Allow the player to repeat his move
    - Else:
      - Move to other player
   
End Phase:
  - Show the end screen which with PLAYER X WON text
  - Provide the same abilities like with pause menu
