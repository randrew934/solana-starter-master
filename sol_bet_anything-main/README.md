# SOL BET ANYTHING(Capstone Project)

### Your Game. Your Rules

Bet anything from Sports games to Event forecasts to Token Predictions to X(Twitter) Banters. All you need is Solana.

## RUST PROGRAM INFORMATION:

**Program ID:** BtYYc5eyu3Eg1WPsJTE3mh1yXFeknwH4xyqhKL8qRUzW 

### USER ROLES:

**User** - Any User

**Maker** - The User that creates a game

**Judge** - The User responsible for ending a game and declaring its winner

**Admin** - The User that sees to the flow of the program

**Signer** (Dual Roles: Admin and Judge)


### STATUS CODE:

**0: Review** - This is the first status code after creating a bet.

**1: Open** - After the Judge validates the bet(accepts their role as a judge), the status code is set to 1, and the bet is open. Setting the Judge as an SBA Judge(admin) automatically sets the status code to 1.

For SBA Judge, the maker is required to submit an advisement which is the same as an appeal after the bet ends.

**2: Judgement** - After the bet period is reached the game ends and the status code is set to 2 if an SBA Judge is not set as the judge.

**3: Appeal** - This is the status code if a game is appealed(The players disagree with the judge declaration)

**4: Declared** - This is the status code after a Judge declares the winner of the bet

**5: Amended** - This is the status code after an SBA Judge(Admin) declares the winner of the bet

**6. Closed** - The status code for the game being closed can’t be appealed or amended anymore. Winners can then withdraw their payouts.

**7. Advisement** - Same as Appeal but it is set so users can help the SBA Judge determine the winner.


### FUNCTIONS:

**inititalize**: Initialize the code. Set the Admin and other admin information. 

**createGame**: Creates the game.

**getGame**: Get Game Information.

**validateGame**: Judge provided accepts its role in the game. If SBA Judge is set, the process is handled automatically. There’s no need to call validateGame.

**placeBet**: Users can get to place a bet on the game.

**endGame**: For the admin to end the game once a bet period has been reached. **declareWinner**: For the judge or admin to declare the winner of the game.

**makeAppeal**: For the user to appeal the declaration of the winner before the game is finally closed.

**getAppeal**: Get Appeal Information.

**closeGame**: After the appeal period has been exceeded. The game is closed by the admin.

**payWinner**: This is used to pay the winner of the game. **changeAdmin**: To change the current admin account. **changeAdminFees**: To change the fees of the Admin.

**withdraw**: To withdraw from the Treasury to the Admin Account

  
