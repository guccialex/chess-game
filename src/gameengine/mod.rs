
mod datastructs;
mod physicsengine;


use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;


use datastructs::PieceData;
pub use datastructs::PieceAction;
use datastructs::PieceType;


pub use datastructs::BoardSquarePosID;
pub use datastructs::RelativeSquare;


use physicsengine::RapierPhysicsWrapper;


use physicsengine::Mission;





#[derive(Serialize, Deserialize)]
pub struct BoardObjects{
    
    boardsquares: HashMap<BoardSquarePosID, u16>,
    
    pieces: HashSet<u16>,
    
    pieceowners: HashMap<u16, u8>,
    
    piecedata: HashMap<u16, PieceData>,
}

impl BoardObjects{
    

    fn get_owners_highest_value_piece(&self, owner: &u8 ) -> (u16, u8){

        let mut highestvalue: i8 = -1;
        let mut highestvalueid: Option<u16> = None;

        for (curpieceid, curowner) in &self.pieceowners{

            if owner == curowner{

                let piecedata = self.piecedata.get( &curpieceid ).unwrap();
                let curvalue = piecedata.get_value() as i8;

                if  curvalue > highestvalue{

                    highestvalueid = Some(*curpieceid);
                    highestvalue = curvalue;
                }
            }
        }


        if let Some(toreturn) = highestvalueid{
            return (toreturn, highestvalue as u8);
        }

        panic!("no pieces for this player");
    }

    
    fn new() -> BoardObjects{
        
        BoardObjects{
            boardsquares: HashMap::new(),
            pieces: HashSet::new(),
            pieceowners: HashMap::new(),
            piecedata: HashMap::new(),
        }
    }
    
    fn add_piece(&mut self, objectid: u16, owner: u8, piecedata: PieceData){
        
        self.pieces.insert( objectid);
        
        self.pieceowners.insert( objectid, owner );
        
        self.piecedata.insert( objectid, piecedata);
    }
    
    
    fn add_boardsquare(&mut self, pos: BoardSquarePosID, objectid: u16){
        self.boardsquares.insert( pos, objectid);
    }
    
    fn get_boardsquares(&self) -> HashSet<BoardSquarePosID>{
        
        let mut toreturn = HashSet::new();
        
        for (bsid, _) in &self.boardsquares{
            toreturn.insert(bsid.clone());
        }
        return toreturn;
    }
    
    fn get_pieces(&self) -> HashSet<u16>{
        
        self.pieces.clone()
    }
    
    fn get_boardsquare_object_id(&self, bsid: &BoardSquarePosID) -> Option<u16>{
        
        self.boardsquares.get( bsid ).copied()
    }

    fn get_boardsquare_by_object_id(&self, objectid: &u16) -> BoardSquarePosID{

        for (curbsid, curobjectid) in &self.boardsquares{

            if curobjectid == objectid{

                return curbsid.clone();
            }
        }

        panic!("bsid not found from this objectid");
    }
    
    fn get_mut_piecedata(&mut self, pieceid: &u16) -> &mut PieceData{
        self.piecedata.get_mut(pieceid).unwrap()
    }

    fn get_piecedata(&self, pieceid: &u16) -> PieceData{

        self.piecedata.get(pieceid).unwrap().clone()
    }

    fn get_players_pieces(&self, playerid: &u8) -> HashSet<u16>{

        let mut toreturn = HashSet::new();

        for (pieceid, owner) in &self.pieceowners{

            if owner == playerid{
                toreturn.insert(*pieceid);
            }
        }

        toreturn
    }

    fn get_owner_of_piece(&self, pieceid: &u16) -> u8{

        for (curpieceid, owner) in &self.pieceowners{

            if curpieceid == pieceid{
                return *owner;
            }
        }

        panic!("piece doesnt have owner or piece not found {:?}", pieceid);
    }

    fn remove_piece(&mut self, pieceid: &u16){

        self.pieces.remove(pieceid);

        self.pieceowners.remove(pieceid);

        self.piecedata.remove(pieceid);
    }

    fn does_player_have_king(&self, playerid: &u8) -> bool{
        
        //for every piece that player owns
        for (pieceid, owner) in &self.pieceowners{

            if owner == playerid{

                let piecedata = self.piecedata.get(&pieceid).unwrap();

                if piecedata.is_this_piecetype(&PieceType::King){
                    return true;
                }   
            }
        }
        
        return false;
    }

    fn does_player_have_pieces(&self, playerid: &u8) -> bool{


        //for every piece that player owns
        for (_, owner) in &self.pieceowners{

            if owner == playerid{

                return true;   
            }
        }
        
        return false;

    }

    fn is_object_boardsquare(&self, objectid: &u16) -> bool{

        for (_, curobjectid) in self.boardsquares.iter(){
            if objectid == curobjectid{
                return true;
            }
        }

        return false;
    }

    fn get_boardsquare_object_ids(&self) -> HashSet<u16>{

        let mut toreturn = HashSet::new();

        for (_, objectid) in &self.boardsquares{

            toreturn.insert(*objectid);
        };

        toreturn
    }


    fn get_object_ids(&self) -> HashSet<u16>{

        let mut toreturn = self.get_pieces();

        toreturn.extend( self.get_boardsquare_object_ids() );
        
        toreturn
    }



    
}


//in order of methods that rely on earlier methods

#[derive(Serialize, Deserialize)]
pub struct GameEngine{
    
    boardobjects: BoardObjects,
    
    //the direction the player i facing, of the 8 cardinal directions
    playertodirection: HashMap<u8, f32>,
    
    physics: RapierPhysicsWrapper,
}



//most baremetal to most abstracted functions

//step 1, game engine functions that dont rely on other game engine functions
//game engine 
impl GameEngine{

    fn new_physical_board() -> RapierPhysicsWrapper{
        
        let mut boardgame = RapierPhysicsWrapper::new();
        
        //create the 4 invisible walls bordering the game
        {
            let horizontalwalldimensions = (20.0 , 20.0 , 4.0);
            let verticalwalldimensions = (4.0 , 20.0 , 20.0 );
            
            let physicalid = boardgame.add_object(true);
            boardgame.set_translation( &physicalid,  (0.0,0.0,-6.0) );
            boardgame.set_shape_cuboid(&physicalid, horizontalwalldimensions );
            
            let physicalid = boardgame.add_object(true);
            boardgame.set_translation( &physicalid,  (0.0,0.0,6.0) );
            boardgame.set_shape_cuboid(&physicalid, horizontalwalldimensions );
            
            let physicalid = boardgame.add_object(true);
            boardgame.set_translation( &physicalid,  (-6.0,0.0,0.0) );
            boardgame.set_shape_cuboid(&physicalid, verticalwalldimensions );
            
            let physicalid = boardgame.add_object(true);
            boardgame.set_translation( &physicalid,  (6.0,0.0,0.0) );
            boardgame.set_shape_cuboid(&physicalid, verticalwalldimensions );
        }
        
        boardgame
    }
    
    fn create_piece_object(&mut self, pos: (f32,f32,f32) ) -> u16{
        
        let objectid = self.physics.add_object(false);
        
        self.physics.set_shape_cylinder(&objectid, 0.5, 0.7 );
        self.physics.set_materials(&objectid, 0.5, 0.5);
        self.physics.set_translation( &objectid, ( pos.0 , pos.1 , pos.2 ) );
        
        objectid
    }
    
    fn create_boardsquare_object(&mut self, pos: (f32, f32, f32) ) -> u16{
        
        let objectid = self.physics.add_object( true );
        
        let ypos = 0.0;
        
        self.physics.set_shape_cuboid(&objectid, (1.0, 1.0, 1.0) );
        self.physics.set_materials(&objectid, 0.0, 0.0);        
        self.physics.set_translation( &objectid, ( pos.0 , pos.1,  pos.2  ) );        
        
        objectid
    }
    
    //get object 1's x&z position relative to object 2's
    fn flat_plane_object_offset(&self, object1: u16, object2: u16 ) -> (f32,f32){
        
        let object1pos = self.physics.get_translation(&object1);
        let object2pos = self.physics.get_translation(&object2);
        
        //get the pieces x and z position and subtract the position of the piece its on from it
        let xoffset = object1pos.0 - object2pos.0;
        let zoffset = object1pos.1 - object2pos.1;
        
        return (xoffset, zoffset);
    }
    
    //is this object in this range of positions?
    fn is_object_in_position_range(&self, objectid: u16, xrange: (f32,f32), yrange: (f32,f32), zrange: (f32,f32) ) -> bool{
        
        //get its position
        let (x,y,z) = self.physics.get_translation( &objectid );
        
        if x >= xrange.0 && x<= xrange.1{
            
            if y >= yrange.0 && y<= yrange.1{
                
                if x>= zrange.0 && z<= zrange.1{
                    
                    return true;
                }
            }
        }
        
        return false;
    }
    
    fn slide_object(&mut self, ticksuntil: u32, objectid: u16, mut relativepos: (f32,f32)){
        
        //slide to the center of a piece
        let slidemission = Mission::make_slide_mission( relativepos );
        self.physics.set_future_mission(ticksuntil, objectid, slidemission);
    }
    
    //flick a piece in a direction (radians), with a force
    fn flick_object(&mut self, objectid: u16, direction: f32, force: f32){
        
        //create a mission
        let flickmission = Mission::make_flick_mission( direction, force);
        self.physics.set_mission(objectid, flickmission );
    }
    
    //lift and move a piece to another position
    fn lift_and_move_object(&mut self, ticksuntil: u32, objectid: u16, mut relativepos: (f32,f32)){
        
        let liftandmovemission = Mission::make_lift_mission( relativepos );
        self.physics.set_future_mission( ticksuntil, objectid, liftandmovemission );
    }
    
    fn set_long_drop(&mut self, length: u32, objectid: u16){
        
        let mut mission = Mission::make_lengthed_drop(length);
        self.mission_set_current_pos_as_default(&objectid, &mut mission);
        self.physics.set_mission(objectid, mission);
    }
    
    fn set_long_raise(&mut self, length: u32, objectid: u16){
        
        let mut mission = Mission::make_lengthed_raise(length);
        self.mission_set_current_pos_as_default(&objectid, &mut mission);
        self.physics.set_mission(objectid, mission);
    }
    
    fn set_future_drop(&mut self, ticks: u32, bsid: u16){
        
        let mut mission = Mission::make_drop_and_loop_around();
        self.mission_set_current_pos_as_default(&bsid, &mut mission);
        self.physics.set_future_mission(ticks, bsid, mission);
    }

}




//game engine functions that rely on step 1 gameengine functions
//deal with the abstractions of the objects as pieces
impl GameEngine{
    
    //set the current position of the object as the default position on the object
    fn mission_set_current_pos_as_default(&self, id: &u16,  mission: &mut Mission){
        
        let pos = self.physics.get_translation(id);
        let rot = self.physics.get_rotation(id);
        mission.set_default_isometry(pos, rot);
    }
    
    fn get_objects_on_mission_of_type(&self, missiontype: &MissionType) -> Vec<u16>{
        
        let mut toreturn = Vec::new();
        
        for (affectedobjectid, mission) in self.physics.get_active_missions(){
            if &mission.mission_type() == missiontype{
                toreturn.push( affectedobjectid);
            }
        }
        
        toreturn        
    }
    
    
    fn is_boardsquare_on_mission(&self, bsid: &BoardSquarePosID) -> bool{
        
        let objectid = self.boardobjects.get_boardsquare_object_id(bsid).unwrap();
        
        return self.physics.is_object_on_mission( &objectid);
    }
    
    //create a piece at this position of this type
    fn create_piece(&mut self, pos: BoardSquarePosID, owner: u8, piecetype: PieceType){
        
        let piecepos = (pos.to_physical_pos().0,  3.0  ,pos.to_physical_pos().1 );
        
        let objectid = self.create_piece_object( piecepos );
        
        let mut piecedata = PieceData::new();
        piecedata.set_piecetype( piecetype);
        
        self.boardobjects.add_piece(objectid, owner, piecedata);
    }
    
    
    fn create_boardsquare(&mut self, bsid: BoardSquarePosID){
        
        let bsidphyspos = (bsid.to_physical_pos().0, 0.0, bsid.to_physical_pos().1 );
        let objectid = self.create_boardsquare_object( bsidphyspos );
        
        self.boardobjects.add_boardsquare(bsid , objectid);
    }
    
    
    fn get_pieces_on_board_square(&self, bsid: &BoardSquarePosID) -> HashSet<u16>{
        
        let mut toreturn = HashSet::new();
        
        
        //for all pieces
        for pieceid in self.boardobjects.get_pieces(){
            
            //get all objects in the range specified
            let range = bsid.get_range_on_self();
            
            if self.is_object_in_position_range(pieceid, range.0, range.1, range.2){
                toreturn.insert( pieceid );
            };
        };
        
        return toreturn;
    }
    
    
    //get the id of every board square without a piece on it
    //and that arent on a mission currently
    fn get_empty_squares_not_on_mission(&self) -> Vec<BoardSquarePosID>{
        
        let bsids = self.boardobjects.get_boardsquares();
        
        
        let mut toreturn = Vec::new();
        
        
        for bsid in bsids{
            
            let piecesonboardsquare = self.get_pieces_on_board_square( &bsid);
            
            //if it doesnt have anything on it
            if piecesonboardsquare.is_empty(){
                
                
                //if its not on a mission
                if ! self.is_boardsquare_on_mission( &bsid) {
                    
                    //then push it into the list of empty squares not on a mission
                    toreturn.push( bsid );
                }
            }
        }
        
        return toreturn;
    }
    
    
    
    pub fn new(player1id: u8, player2id: u8) -> GameEngine{
        
        let mut gameengine = GameEngine{
            playertodirection: HashMap::new(),
            boardobjects: BoardObjects::new(),
            physics: GameEngine::new_physical_board(), 
        };
        
        
        //make the boardsquares
        for x in 0..8{
            for y in 0..8{
                gameengine.create_boardsquare( BoardSquarePosID::new((x,y)).unwrap() );
            }
        }
        
        
        gameengine.playertodirection.insert(player1id, 0.0 );
        gameengine.playertodirection.insert(player2id, 0.5 );
        
        gameengine
    }
    
    


    fn remove_piece(&mut self, pieceid: u16){
        
        self.boardobjects.remove_piece(&pieceid);
        self.physics.remove_object(&pieceid);
    }
    
    
    pub fn get_owner_of_piece(& self, pieceid: u16) -> u8{
        
        return self.boardobjects.get_owner_of_piece( &pieceid );
    }
    
    
    //add the pieces to the game that a chess game would have
    pub fn add_chess_pieces(&mut self){
        
        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            let perspective = *self.playertodirection.get(&playerx).unwrap();
            
            
            for x in 0..8{
                self.create_piece(
                    BoardSquarePosID::new_from_perspective((x, 1), perspective).unwrap(),
                    playerx,
                    PieceType::Pawn
                );
            }
            
            
            self.create_piece(
                BoardSquarePosID::new_from_perspective((0, 0), perspective).unwrap(),
                playerx,
                PieceType::Rook
            );
            self.create_piece(
                BoardSquarePosID::new_from_perspective((1, 0), perspective).unwrap(),
                playerx,
                PieceType::Knight
            );
            self.create_piece(
                BoardSquarePosID::new_from_perspective((2, 0), perspective).unwrap(),
                playerx,
                PieceType::Bishop
            );
            
            
            //swap position of queen and king
            self.create_piece(
                BoardSquarePosID::new_from_perspective((3, 0), perspective).unwrap(),
                playerx,
                PieceType::Queen
            );
            self.create_piece(
                BoardSquarePosID::new_from_perspective((4, 0), perspective).unwrap(),
                playerx,
                PieceType::King
            );
            
            
            self.create_piece(
                BoardSquarePosID::new_from_perspective((5, 0), perspective).unwrap(),
                playerx,
                PieceType::Bishop
            );
            self.create_piece(
                BoardSquarePosID::new_from_perspective((6, 0), perspective).unwrap(),
                playerx,
                PieceType::Knight
            );
            self.create_piece(
                BoardSquarePosID::new_from_perspective((7, 0), perspective).unwrap(),
                playerx,
                PieceType::Rook
            );
        };
    }
    
    //add the pieces to the game that a chess game would havef
    pub fn add_checkers_pieces(&mut self){
        
        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            let perspective = *self.playertodirection.get(&playerx).unwrap();
            
            for x in 0..8{
                
                for z in 0..3{
                    
                    if (x + z) % 2 == 1{
                        
                        self.create_piece(
                            BoardSquarePosID::new_from_perspective((x, z), perspective).unwrap(),
                            playerx,
                            PieceType::Checker
                        );
                    }
                }
            }
        };
    }
    
    
    pub fn get_board_square_piece_is_on(&self, pieceid: &u16) -> Option<BoardSquarePosID>{

        let piecepos = (self.physics.get_translation(pieceid).0, self.physics.get_translation(pieceid).2);

        BoardSquarePosID::from_physical_pos( piecepos )
    }
    
    
    
    //set the number of squares raised
    pub fn set_randomly_raised_squares(&mut self, numbertoraise: u32){
        
        //get the number of raised squares
        let mut curraisedsquares = self.get_objects_on_mission_of_type(  &MissionType::LongRaise );
        
        //how many more raised squares I have than I need
        let difference = curraisedsquares.len() as i32 - numbertoraise as i32;
        
        let absdifference = difference.abs() as usize;
        
        
        if difference > 0{
            
            for x in 0..absdifference{
                
                if let Some(objectid) = curraisedsquares.pop(){

                    self.physics.end_mission( &objectid );
                }
            }
        }
        else if difference < 0{
            
            let mut potentialsquares = self.get_empty_squares_not_on_mission();
            
            for x in 0..absdifference{
                
                if let Some(bsposid) = potentialsquares.pop(){
                    
                    let objectid = self.boardobjects.get_boardsquare_object_id( &bsposid ).unwrap();
                    
                    self.set_long_raise(10000, objectid);
                }
            }
        }
    }
    
    //set the number of squares that should be randomly dropped
    pub fn set_randomly_dropped_squares(&mut self, numbertodrop: u32){
    }
    

    //get each players highest valued piece
    //turn it into as many pawns as that piece was valued
    pub fn split_highest_piece_into_pawns(&mut self){
        
        for playerid in 1..3{
            
            let (highestpieceid, mut highestpiecevalue) = self.boardobjects.get_owners_highest_value_piece(&playerid);
            
            //remove that highest valued piece
            self.remove_piece( highestpieceid );
            

            let mut emptysquares = self.get_empty_squares_not_on_mission();
            

            //create as many pawn pieces as that highest value pieces value is
            for x in 0..highestpiecevalue{
                
                if let Some(bsid) = emptysquares.pop(){

                    self.create_piece( bsid, playerid, PieceType::Pawn );
                }
            }
        }
    }
    
    //give all pieces with a value greater than 1 the ability of knights
    pub fn knightify(&mut self){

        for pieceid in self.boardobjects.get_pieces(){

            self.boardobjects.get_mut_piecedata( &pieceid ).augment_knight_abilities();
        }
    }
    
    pub fn unaugment_abilities(&mut self){
        
        for pieceid in self.boardobjects.get_pieces(){

            self.boardobjects.get_mut_piecedata( &pieceid ).remove_ability_augmentations();
        }
    }
    
    pub fn checkerify(&mut self){
        
        
        for playerid in 1..3{
            
            //get the sum of the value of the players pieces and remove them
            
            let mut valuesum = 0;
            
            for pieceid in self.boardobjects.get_players_pieces(&playerid){
                
                let curvalue = self.boardobjects.get_mut_piecedata(&pieceid).get_value();
                valuesum += curvalue;
                self.remove_piece(pieceid);
            };
            
            
            
            let mut emptysquares = self.get_empty_squares_not_on_mission();

            //create half as many checkers pieces as that players total value of pieces
            for x in 0.. valuesum/2 +1 {
                
                if let Some(bsposid) = emptysquares.pop(){

                    self.create_piece(bsposid, playerid, PieceType::Checker);
                }
            };
        };
    }
    
    pub fn chessify(&mut self) {
        
        for playerid in 1..3{
            
            //get the sum of the value of the players pieces and remove them
            
            let mut valuesum = 0;
            
            for pieceid in self.boardobjects.get_players_pieces(&playerid){
                
                let curvalue = self.boardobjects.get_mut_piecedata(&pieceid).get_value();
                valuesum += curvalue;
                self.remove_piece(pieceid);
            };
            
            
            
            let mut emptysquares = self.get_empty_squares_not_on_mission();

            //create a king first
            if let Some(bsposid) = emptysquares.pop(){
                self.create_piece(bsposid, playerid, PieceType::King);
            }

            for x in 0.. valuesum/2 +1 {                
                if let Some(bsposid) = emptysquares.pop(){
                    self.create_piece(bsposid, playerid, PieceType::Pawn);
                }
            };
        };
    }
    
    //tick, with true if kings are replaced and false if theyre not
    pub fn tick(&mut self, arekingsreplaced: bool, arepawnspromoted: bool){
        
        
        //remove the pieces that are lower than -5 in pos
        for pieceid in &self.boardobjects.get_pieces().clone(){
            
            let pos = self.physics.get_translation(pieceid);
            
            if pos.1 < -4.0{          
                self.remove_piece(*pieceid);
            }
        }
        
        
        
        //if the kings are replaced, the piece with the highest score becomes a king
        if arekingsreplaced{
            
            for playerid in 1..3{
                
                //if they dont
                if ! self.does_player_have_king(&playerid){
                    
                    let (pieceid, _) = self.boardobjects.get_owners_highest_value_piece(&playerid);
                    
                    self.boardobjects.get_mut_piecedata(&pieceid).set_piecetype(PieceType::King);
                }
            }
        }
        
        
        //promote the pawns to queens if theyre on the backrow of their opponent
        if arepawnspromoted{
            
            for pieceid in self.boardobjects.get_pieces(){
                
                //get the owner
                let ownerid = self.boardobjects.get_owner_of_piece( &pieceid);
                
                //get the "objective back row" from that players perspective
                let backrow = GameEngine::subjective_row_to_objective_row(&ownerid, &7);
                
                if let Some( bsposid ) = self.get_board_square_piece_is_on( &pieceid){
                    
                    //if that pawn is on the backrow
                    if bsposid.get_row() == backrow{
                        self.boardobjects.get_piecedata(&pieceid).set_piecetype( PieceType::Queen);
                    }
                }
            }
        }
        
        
        self.physics.tick();
    }    
    

    
    
    
    //get if this action is allowed by this piece
    pub fn is_action_allowed(&self, action: &PieceAction, pieceid: &u16) -> bool{
        
        //get the owner of this piece
        let owner = self.get_owner_of_piece(*pieceid);
        
        //the direction of the owner
        let ownerdirection = self.playertodirection.get(&owner).unwrap();
        
        //if this is is one of the actions the piece is allowed to perform
        let piecedata = self.boardobjects.get_piecedata(&pieceid);
        
        
        //get the square that its on
        if let Some(squareposid) = self.get_board_square_piece_is_on(pieceid){
            
            //if the action is allowed by the piecedata
            if let Some(squareconditions) = piecedata.is_action_valid(action, ownerdirection){
                
                
                //for every square and condition for that square
                for (relativesquare, squarecondition) in squareconditions{
                    
                    //if that square exists
                    if let Some(cursquarepos) = squareposid.new_from_added_relative_pos( relativesquare){
                        
                        
                        use datastructs::SquareCondition;
                        
                        
                        //get whats on the square
                        let piecesonsquare = self.get_pieces_on_board_square(&cursquarepos);
                        
                        
                        
                        match squarecondition{
                            
                            //if the square needs to be empty
                            SquareCondition::EmptyRequired => { 
                                
                                if ! piecesonsquare.is_empty(){
                                    return false;
                                };
                            },
                            //if the square cant have a friendly piece on it
                            SquareCondition::NoneFriendlyRequired =>{
                                
                                //for every piece on the square
                                for otherpieceid in piecesonsquare{
                                    
                                    if self.get_owner_of_piece(*pieceid) == self.get_owner_of_piece(otherpieceid){
                                        return false;
                                    };    
                                };
                            },
                            //if there needs to be at least one opponents piece on this square
                            SquareCondition::OpponentRequired =>{
                                
                                let mut opponentspiece = false;
                                
                                //for every piece on the square
                                for otherpieceid in piecesonsquare{
                                    
                                    if self.get_owner_of_piece(*pieceid) != self.get_owner_of_piece(otherpieceid){
                                        opponentspiece = true;
                                    };
                                };
                                
                                if opponentspiece == false{
                                    return false;
                                };
                            },
                        };
                    }
                    //if the boardsquare targeted isnt valids
                    else{
                        return false;
                    };
                };
            }
            //if the action isnt allowed by the piecedata
            else{
                return false;
            };
        }
        //if its not on a square
        else{
            return false;
        };
        
        
        //if all the actions conditions were met, or none of them werent met
        return true;
    }
    
    
    pub fn perform_action(&mut self, piece: u16, pieceaction: PieceAction ){


        
        
        if let Some(liftandmoveforces) = pieceaction.get_lift_and_move_forces(){

            self.lift_and_move_object(10, piece, liftandmoveforces.to_relative_float() );
        };
        
        if let Some(slideforces) = pieceaction.get_slide_forces(){

            self.slide_object(30, piece, slideforces.to_relative_float() );
        };
        
        if let Some( (direction, force) ) = pieceaction.get_flick_forces(){
            self.flick_object(piece, direction, force);
        };
        
        
        //drop the boardsquares that should be dropped when they should be dropped
        for (squareposrelative, tick) in pieceaction.get_squares_dropped_relative(){
            
            let squareposid = self.get_board_square_piece_is_on(&piece).unwrap();
            
            if let Some(bsid) = squareposid.new_from_added_relative_pos( squareposrelative ){
                
                let relativesquareid = self.boardobjects.get_boardsquare_object_id( &bsid ).unwrap();
                
                self.set_future_drop(tick, relativesquareid);
            };
        };
        
        
        //set the piece has having moved
        self.boardobjects.get_piecedata(&piece).moved_piece();

    }
    
    
    
    //get the list of every object in the physical engine
    pub fn get_object_ids(&self) -> HashSet<u16>{
        self.boardobjects.get_object_ids()
    }
    
    pub fn does_player_have_king(&self, playerid: &u8) -> bool{
        self.boardobjects.does_player_have_king(playerid)
    }

    pub fn does_player_have_pieces(&self, playerid: &u8) -> bool{
        self.boardobjects.does_player_have_pieces(playerid)
    }
    
    
    
    
    
    //get the row from a players perspective (0 is closest row to player, 7 is farther row from player)
    //and returns what row that is
    fn subjective_row_to_objective_row(playerid: &u8, subjectiverow: &i8) -> i8{
        
        if playerid == &1{
            
            return *subjectiverow;
        }
        else if playerid == &2{
            
            return 7 - subjectiverow;
        }
        else{
            panic!("no player other than 1 and 2");
        }
    }
    
    
    

    
}









//getters used only outside of this module
impl GameEngine{
    
    
    pub fn is_object_on_mission(&self, id: u16) -> bool{
        
        self.physics.is_object_on_mission(&id)
    }
    
    
    //get the pieces that are targeted by a piece performing an action
    fn get_piece_targets_of_action(&self, pieceid: &u16, action: &PieceAction) -> Vec<u16>{
        
        let mut toreturn = Vec::new();
        
        //get the boardsquares dropped by this action
        
        //get the pieces on those boardsquares
        
        if let Some(boardsquareid) = self.get_board_square_piece_is_on(pieceid){
            
            for (relativeposid, _) in action.get_squares_dropped_relative(){
                
                if let Some(newboardsquare) = boardsquareid.new_from_added_relative_pos( relativeposid){
                    
                    toreturn.extend( self.get_pieces_on_board_square( &newboardsquare )  );
                };
            };
        };
        
        toreturn        
    }
    
    //get the action that this piece can perform now, and the objects it targets
    pub fn get_piece_valid_actions_and_targets(&self, pieceid: &u16) -> (bool, Vec< (PieceAction, Vec<u16>) >){
        
        
        //get the piece data
        let piecedata = self.boardobjects.get_piecedata(pieceid);
        
        //the owner of the piece
        let owner = self.boardobjects.get_owner_of_piece(pieceid);

        //the direction of the owner of the piece
        let ownerdirection = self.playertodirection.get(&owner).unwrap();
        
        //get all the actions this piece can potentially perform
        let allactions = piecedata.get_numberable_piece_actions(ownerdirection);
        
        
        let mut actionsandtargets: Vec<(PieceAction, Vec<u16>)> = Vec::new();
        
        
        //for every action, get if it is allowed
        for action in allactions{
            
            if self.is_action_allowed(&action.clone(), &pieceid){
                
                let mut targets = self.get_piece_targets_of_action(&pieceid, &action);
                
                let curbsposid = self.get_board_square_piece_is_on(pieceid).unwrap();
                
                if let Some(targetbsposid) = curbsposid.new_from_added_relative_pos( action.get_relative_position_action_takes_piece() ){
                    
                    targets.push( self.boardobjects.get_boardsquare_object_id(&targetbsposid).unwrap() );                    
                }                
                
                actionsandtargets.push( (action, targets) );
            };
        };
        
        
        let flickable = self.is_action_allowed( &PieceAction::flick(1.0, 1.0) , &pieceid);
        
        return (flickable, actionsandtargets);        
    }
    
    //is this board game object a square
    pub fn is_board_game_object_square(&self, objectid: &u16) -> bool{

        self.boardobjects.is_object_boardsquare(objectid)
    }
    
    //is this board game object a piece
    pub fn is_board_game_object_piece(&self, objectid: &u16) -> bool{

        self.boardobjects.get_pieces().contains(objectid)
    }
    
    //get the name of the type of the piece
    pub fn get_piece_image_location(&self, pieceid: u16) -> String{
        
        let piecetypedata = self.boardobjects.get_piecedata(&pieceid);
        
        piecetypedata.get_image_location()
    }
    
    pub fn is_boardsquare_white(&self, bsid: &u16 ) -> bool{
        
        let bspos = self.boardobjects.get_boardsquare_by_object_id(bsid).get_pos();
        
        let bstotal = bspos.0 + bspos.1;
        
        let evenness = bstotal % 2;
        
        if evenness == 0{
            return true;
        }
        else{
            return false;
        }
    }
    
    //its translation and its rotation
    pub fn get_object_isometry(&self, gameobjectid: &u16) -> ((f32,f32,f32), (f32,f32,f32)){
        
        (self.physics.get_translation(gameobjectid), self.physics.get_rotation(gameobjectid))
    }
}











//mission implementations
impl Mission{
    
    
    //make the mission of flicking a piece
    pub fn make_flick_mission(direction: f32, force: f32) -> Mission{
        
        let mut toreturn = Mission::default_mission( MissionType::Flick.to_number() );
        
        //add impulse 
        toreturn.add_impulse_change( 0,1, (direction.cos()*force, 0.0 , direction.sin()*force) );
        
        toreturn
    }
    
    
    //for pieces
    pub fn make_lift_mission(relativepos: (f32,f32)) -> Mission{
        
        let mut toreturn = Mission::default_mission( MissionType::LiftAndMove.to_number() );
        
        //the timesteps at which the states change
        let lifttomove = 10;
        let movetodrop = 20;
        let endtick = 30;
        
        
        toreturn.add_position_change(0, lifttomove, (0.0, 0.1, 0.0) );
        
        let totalmoveticks = movetodrop - lifttomove;
        let xchangepertick = relativepos.0 / (totalmoveticks) as f32;
        let zchangepertick = relativepos.1 / (totalmoveticks) as f32;
        toreturn.add_position_change(lifttomove, movetodrop,(xchangepertick, 0.0, zchangepertick) );

        toreturn.add_position_change(movetodrop, endtick, (0.0, -0.1, 0.0) );
        
        toreturn
    }
    
    //make a slide mission given the relative position for the piece to slide to
    pub fn make_slide_mission(relativepos: (f32,f32)) -> Mission{
        
        let mut toreturn = Mission::default_mission( MissionType::Slide.to_number() );
        
        //get the distance so i can determine how long to make the slide
        let slidedistance = (relativepos.0 * relativepos.0 + relativepos.1 * relativepos.1).sqrt();
        
        //the total amount of ticks
        let ticks = (slidedistance * 5.0).ceil() as u32;
        
        let xchangepertick = relativepos.0 / (ticks) as f32;
        let zchangepertick = relativepos.1 / (ticks) as f32;
        
        toreturn.add_position_change(0, ticks, (xchangepertick, 0.0, zchangepertick));    
        
        toreturn
    }
    
    
    //a mission for a boardsquare that drops it then makes it sink from the top back to teh bottom
    pub fn make_drop_and_loop_around() -> Mission{
        
        
        let mut toreturn = Mission::default_mission( MissionType::ShortDrop.to_number() );
        
        
        //the object stops dropping
        //starts moving to the left
        let enddrop = 3;
        //the object stops moving to the left
        //starts raising
        let endleft = 6;
        //the object raises up
        let endraise = 9;
        //the object comes back to where it was
        let endright = 12;
        //the object shoots back down into its original position
        let endrestore = 21;
        
        
        toreturn.add_position_change(0, enddrop,  (0.0, -1.5, 0.0)   );
        toreturn.add_position_change(enddrop, endleft, (-6.0, 0.0, 0.0)  );
        toreturn.add_position_change(endleft, endraise, (0.0, 3.0, 0.0)  );
        toreturn.add_position_change(endraise, endright, (6.0, 0.0, 0.0)  );
        toreturn.add_position_change( endright, endrestore, (0.0, -0.50, 0.0) );
        
        
        toreturn
    }
    
    
    pub fn make_lengthed_drop(ticks: u32) -> Mission{
        
        let mut toreturn = Mission::default_mission( MissionType::LongDrop.to_number() );
        
        //when the object stops dropping
        let enddrop = 5;
        let waitstillend = 5 + ticks;
        let restoreend = waitstillend + 5;
        
        
        //lower
        toreturn.add_position_change(0, enddrop, (0.0, -2.0, 0.0) );
        
        //wait
        toreturn.add_position_change(enddrop, waitstillend, (0.0, 0.0, 0.0) );        
        
        //return back to its original position
        toreturn.add_position_change(waitstillend, restoreend, (0.0, 2.0, 0.0) );
        
        toreturn
    }
    
    
    pub fn make_lengthed_raise(ticks: u32) -> Mission{
        
        let mut toreturn = Mission::default_mission(  MissionType::LongRaise.to_number() );
        
        //when the object stops dropping
        let endraise = 5;
        let wait = 5 + ticks;
        let restore = 5 + ticks + 5;
        
        toreturn.add_position_change(  0, endraise, (0.0, 0.2, 0.0)     );
        
        toreturn.add_position_change(  endraise, wait, (0.0, 0.0, 0.0)  );
        
        toreturn.add_position_change(  wait, restore, (0.0, -0.2, 0.0)  );
        
        toreturn
    }
    
    //return the type of mission it is
    pub fn mission_type(&self) -> MissionType {
        MissionType::from_number(  self.get_mission_data() )
    }
    
}






//the player implements the 
//the physics engine needs an object that implements

/*
tick
get current impulse
get current delta position
is finished
*/

//and the user implements it
//and then passes it into the engine

//the types of missions
#[derive(PartialEq)]
pub enum MissionType{
    
    LongDrop,
    
    LongRaise,
    
    ShortDrop,
    
    Slide,
    
    
    LiftAndMove,
    
    Flick,
    
}

impl MissionType{
    
    
    fn to_number(&self) -> u16{
        
        match *self{
            MissionType::LongDrop => 0,
            MissionType::LongRaise => 1,
            MissionType::ShortDrop => 2,
            MissionType::Slide => 3,
            MissionType::LiftAndMove => 4,
            MissionType::Flick => 5,
        }
    }
    
    fn from_number(number: u16) -> MissionType{
        
        match number{
            0 => MissionType::LongDrop,
            1 => MissionType::LongRaise,
            2 => MissionType::ShortDrop,
            3 => MissionType::Slide,
            4 => MissionType::LiftAndMove,
            5 => MissionType::Flick,
            _ => panic!("what number is this?"),
        }
    }
    
    
    
}


