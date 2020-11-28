
import init, { FullGame } from './wasmfiles/wasm_builder.js';



let websocketaddress = 'ws://localhost:3012';



run();


/*
the server:
opens a websocket connection with the game
waits for "gametoconnectto" information from the client
-then sends a message that its in a game but is waiting for another player to make the game start-
-then sends a message that its in a game and the other player is connected, and its starting-

then it receives a message from each connected player whenever they makes a move, a serialized "playerinput"

and it sends, periodically, the state of the game on the server to the connected clients




the client:
opens a websocket connection with the server
sends a "gametoconnectto" message to the server
waits until it gets a message that it is in a game waiting for another player
waits until it gets a message that it is in a game and is connected to another player and the game started





messages I need to serialize to send between server and client:

-> the state of the game the client is in:
-is waiting for user to input what type of game to connect to
-is in a game and waiting for another player to connect to the game for it to start
-game started (and the state of the chess checker game)


-> player input






the wasm
this has the state of the game:
whether its in a game
waiting for user input on what type of game


the way the game interfaces with the wasm:

the game can give the wasm
-player input
-websocket message from the server

the wasm gives the game
-state of the game
-websocket message to the server


*/



async function run() {
    
    
    await init();
    
    
    
    //create a websocket connection with the server
    let socket = new WebSocket( websocketaddress );
    
    
    //when connected to the server, start the game
    socket.addEventListener('open', function (event) {
        
        
        start(socket);
        
        
    });
    
    
    
    
    
}




async function start(socket){
    
    
    let canvas = document.getElementById("renderCanvas"); // Get the canvas element
    let engine = new BABYLON.Engine(canvas, true); // Generate the BABYLON 3D engine
    
    
    let mygame = new GameInterface(engine, socket);
    console.log(mygame);
    
    
    
    
    
    //create an event listener that when a message is received, it is sent to the game
    //(which should have been created, because a message cant be received if the connection hasnt been established)
    mygame.socket.addEventListener('message', function (event) {
        
        console.log("GOT MESSAGE");
        mygame.get_message(event.data);
        
    });
    
    
    
    rungame(mygame);
    
    
    
    
}




async function rungame(thegame) {
    
    console.log("STARtING GAME");
    
    //run the tick function of the game 30 times per second
    thegame.gameappearance.engine.runRenderLoop(function () {
        
        thegame.tick();
        
    });
    
    
}



//the appearance of the game state
//doesnt this also manage getting input?
class GameApperance{
    
    constructor(engine, gameinterface){
        
        //create a scene for the engine
        let scene = new BABYLON.Scene(engine);
        
        this.engine = engine;
        
        
        // This creates and positions a free camera (non-mesh)
        var camera = new BABYLON.FreeCamera("camera1", new BABYLON.Vector3(0, 5, -10), scene);
        
        // This targets the camera to scene origin
        camera.setTarget(BABYLON.Vector3.Zero());        
        
        //get the canvas for this engine to attach a control tos
        let canvas = engine.getRenderingCanvas();
        camera.attachControl(canvas, true);
        
        // This creates a light, aiming 0,1,0 - to the sky (non-mesh)
        var light = new BABYLON.HemisphericLight("light1", new BABYLON.Vector3(0, 1, 0), scene);
        
        
        
        
        this.advancedTexture = BABYLON.GUI.AdvancedDynamicTexture.CreateFullscreenUI("UI");
        
        
        this.thegameinterface = gameinterface;
        
        this.scene = scene;
        
        
        //if this appearance object is in a connecting state
        this.is_in_connecting_state = false;
        
        //if this appearance object is in a game state
        this.is_in_game_state = false;
        
        
        
    }
    
    
    //set the state of the game to get the information about how to connect to the game from the user
    set_get_connection_information_state(){
        
        this.is_in_connecting_state = true;
        this.is_in_game_state = false;
        
        
        
        var privategameidinput = new BABYLON.GUI.InputText();
        privategameidinput.top = "20%"
        privategameidinput.left = "-5%"
        privategameidinput.height = "40px";
        privategameidinput.width = "100px";
        privategameidinput.text = "game ID";
        privategameidinput.color = "white";
        privategameidinput.background = "green";
        this.advancedTexture.addControl(privategameidinput);
        
        
        //the id of the text to join
        this.privategameid = privategameidinput;
        
        
        var connecttopublicgame = new BABYLON.GUI.Button.CreateSimpleButton("but1", "join a public game");
        //when this game wants to connect to an open public game
        connecttopublicgame.onPointerClickObservable.add( () => { this.connect_to_public_game(); });
        connecttopublicgame.top = "-20%";
        connecttopublicgame.height = "50px";
        connecttopublicgame.left = "5%";
        connecttopublicgame.width = "140px";
        connecttopublicgame.color = "white";
        connecttopublicgame.background = "green";
        this.advancedTexture.addControl(connecttopublicgame);
        
        
        var createnewprivategame = new BABYLON.GUI.Button.CreateSimpleButton("but1", "create a new private game");
        let tempvalue = this.thegameinterface;
        createnewprivategame.onPointerClickObservable.add( () => {    this.connect_to_new_private_game();    });
        //this.thegameinterface.connect_to_new_private_game();
        createnewprivategame.top = "0%";
        createnewprivategame.height = "50px";
        createnewprivategame.left = "5%";
        createnewprivategame.width = "140px";
        createnewprivategame.color = "white";
        createnewprivategame.background = "green";
        this.advancedTexture.addControl(createnewprivategame);
        
        
        
        var connecttoprivategame = new BABYLON.GUI.Button.CreateSimpleButton("but1", "connect to a private game");
        
        
        connecttoprivategame.onPointerClickObservable.add( () => {this.connect_to_existing_private_game() } );
        
        connecttoprivategame.top = "20%";
        connecttoprivategame.height = "50px";
        connecttoprivategame.left = "5%";
        connecttoprivategame.width = "140px";
        connecttoprivategame.color = "white";
        connecttoprivategame.background = "green";
        
        this.advancedTexture.addControl(connecttoprivategame);
        
        
        
        
    }
    
    //set the state of the scene when it becomes a game state (getting rid of the buttons and shit)
    set_game_state(){
        
        this.is_in_game_state = true;
        this.is_in_connecting_state = false;
        
        
    }
    
    
    //render the scene using the appearance data
    render(appearancedata){
        
        //if its in a connection state, and the game appearane is not in a connecting state
        if (appearancedata.is_in_connecting_state && ! this.is_in_connecting_state){
            this.set_get_connection_information_state();
        }
        //if its in a game state, start processing the game data
        if (appearancedata.is_in_game_state && ! this.is_in_game_state){
            this.set_game_state();
        }
        
        
        
        //for each object in the appearance data
        for (let objectdata of appearancedata.objects){
            

            console.log(objectdata);
            /*
            //get the name of the mesh of the piece
            let boardsquarename = BoardSquareIDtoName(boardsquareid);
            
            //get the mesh if it exists
            let objectmesh = this.scene.getMeshByName(boardsquarename);
            
            
            //if the mesh doesnt exist, create it first
            if (objectmesh == null){
                
                
                objectmesh = BABYLON.MeshBuilder.CreateBox(boardsquarename, {height: 1, width: 1, depth: 1 }, this.scene);
                
                
                if ((boardsquareid[0] + boardsquareid[1]) %2 == 0){   
                    objectmesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
                    objectmesh.material.diffuseColor = BABYLON.Color3.White();
                }
                else{
                    objectmesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
                    objectmesh.material.diffuseColor = BABYLON.Color3.Gray();
                }
                
                
                
                
                console.log("its a new object after all");
            }
            
            
            
            let boardsquaredata = this.game.get_board_square_data(boardsquareid[0], boardsquareid[1]);
            
            //set the position of the piece
            
            objectmesh.position.x = boardsquaredata.xposition;
            objectmesh.position.y = boardsquaredata.yposition;
            objectmesh.position.z = boardsquaredata.zposition;
            
            objectmesh.rotation.x = boardsquaredata.xrotation;
            objectmesh.rotation.y = boardsquaredata.yrotation;
            objectmesh.rotation.z = boardsquaredata.zrotation;
            */
            
            
            
        }
        
        
        
        
        this.scene.render();
        
    }
    
    
    //the functions that need to be called during events and so have to be functions in this class
    connect_to_existing_private_game(){
        
        this.thegameinterface.connect_to_existing_private_game(this.privategameid.text);
        
    }
    
    connect_to_new_private_game(){
        
        this.thegameinterface.connect_to_new_private_game();
        
    }
    
    connect_to_public_game(){
        
        this.thegameinterface.connect_to_public_game( );
        
        
    }
    
    
    
}



//this class is called when the player creates a new game
class GameInterface{
    
    
    
    //get an engine
    constructor(engine, socket){
        
        //create the "appearance" object for this game, giving it the scene of the engine
        this.gameappearance = new GameApperance(engine, this);
        
        this.socket = socket;
        
        //create the wasm game
        this.wasmgame = FullGame.new();
        
    }
    
    
    
    //get a websocket message from the server
    get_message(message){
        
        console.log("receiving a message from the server", message);
        
        //give the received message to the game
        this.wasmgame.get_incoming_socket_message( message );
    }
    
    
    //render the scene
    render(){
        
        //get appearance data and send it to the GameAppearance object to render
        let appearancedata = this.wasmgame.get_appearance_data();
        //console.log(appearancedata);
        this.gameappearance.render(appearancedata);
    }
    
    
    tick() {
        
        
        //tick the internal game
        this.wasmgame.tick();
        
        //render it
        this.render();
        
        
        //get if any outgoing message is queued to be sent
        if (this.wasmgame.is_outgoing_socket_message_queued() ){
            
            console.log("im sending a websocket message");
            
            //send them to the server
            this.socket.send( this.wasmgame.pop_outgoing_socket_message() );
        }
        
        
    }
    
    
    
    
    //used as the functions attached to the "GameAppearance"
    //to call when there is certain player input
    connect_to_public_game(){
        
        //get the this.wasmgame to set in its list of messages to send
        //to send a message that says it wants to connect to a public game
        this.wasmgame.connect_to_public_game();
        
    }
    
    connect_to_new_private_game(){
        
        //get the this.wasmgame to set in its list of messages to send
        //to send a message that says it wants to connect to a new private game
        
        this.wasmgame.connect_to_new_private_game();
        
    }
    
    connect_to_existing_private_game(privategameid){
        
        
        //get the this.wasmgame to set in its list of messages to send
        //to send a message that says it wants to connect to an existing private game
        
        this.wasmgame.connect_to_existing_private_game(privategameid);
        
    }
    
    
    
    
    
}