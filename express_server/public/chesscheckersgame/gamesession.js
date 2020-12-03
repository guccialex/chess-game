
//when matched up to be put in a game


import init, { FullGame } from './wasmfiles/wasm_builder.js';



//these are gotten from outside or the calling function


let websocketaddress = 'ws://localhost:3012';

//whether this is player 1 or 2
let playerid = 1;

//the password for the game that needs to be given to the server
let gamepassword = "somepassword";




run();




async function run() {
    
    await init();
    
    
    //create a websocket connection with the server
    let socket = new WebSocket( websocketaddress );
    
    
    
    socket.onopen = function (event) {
        
        //when connected, send a message with the password
        socket.send( gamepassword );
        
    };
    
    
    
    socket.onmessage = function (event) {
        
        console.log("connected to game");
        
        //if its a message that im connected to the game
        if (event.data == "connected to game"){
            
            
            //remove the "onmessage "event listener
            socket.onmessage = null;
            
            
            //start the game and give it the socket connection with the server
            start(socket);
            
            
        }
        
        
    };
    
    
    
    
    
    
    
    
}





async function start(socket){
    
    
    let canvas = document.getElementById("renderCanvas"); // Get the canvas element
    let engine = new BABYLON.Engine(canvas, true); // Generate the BABYLON 3D engine
    
    let mygame = new GameInterface(engine, socket);
    
    console.log("started");
    
    
    
    
    
    
    
    //create an event listener that when a message is received, it is sent to the game
    mygame.socket.onmessage = function (event) {
        
        mygame.get_message(event.data);
        
    };
    
    
    
    //run the game
    rungame(mygame);
    
}







async function rungame(thegame) {
    
    console.log("STARtING GAME");
    
    
    
    //add an event listener for the mouse going up
    window.addEventListener("click", function () {
        
        thegame.mouseup();
        
    });
    
    
    //add an event for themouse going down
    window.addEventListener("pointerdown", function () {
        
        thegame.mousedown();
        
    });
    
    //add an event for themouse moving
    window.addEventListener("pointermove", function () {
        
        thegame.mousemove();
        
    });
    
    
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
        let camera = new BABYLON.ArcRotateCamera("camera1", 0, 0, 0, new BABYLON.Vector3(0.0,2.0,0.0), scene);
        
        //set the position of the camera, not its target tho
        camera.setPosition(new BABYLON.Vector3(0, 15, -7));
        
        camera.lowerBetaLimit = 0.1;
        camera.upperBetaLimit = (Math.PI / 2) * 1.0;
        
        camera.lowerRadiusLimit = 10;
        camera.upperRadiusLimit = 30;
        
        
        
        
        //get the canvas for this engine to attach a control tos
        let canvas = engine.getRenderingCanvas();
        
        
        camera.attachControl(canvas, true);
        camera.inputs.attached["mousewheel"].wheelPrecision = 10;
        camera.inputs.attached.keyboard.detachControl();
        
        
        // This creates a light, aiming 0,1,0 - to the sky (non-mesh)
        var light = new BABYLON.HemisphericLight("light1", new BABYLON.Vector3(0, 1, 0), scene);
        
        
        
        
        this.advancedTexture = BABYLON.GUI.AdvancedDynamicTexture.CreateFullscreenUI("UI");
        
        
        this.thegameinterface = gameinterface;
        
        this.scene = scene;
        
        this.camera = camera;
        
        
        
        //create the plane
        let mesh = BABYLON.MeshBuilder.CreateBox("plane", {height: 0.008, width: 100.98, depth: 100.08 }, this.scene);
        mesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
        mesh.material.alpha = 0.25;
        mesh.material.diffuseColor = BABYLON.Color3.Gray();
        mesh.position.y = 1.1;
        
        
        
        var skybox = BABYLON.Mesh.CreateBox("skybox", 500.0, this.scene);
        var skyboxMaterial = new BABYLON.StandardMaterial("skybox", this.scene);
        skyboxMaterial.backFaceCulling = false;
        skyboxMaterial.reflectionTexture = new BABYLON.CubeTexture("skybox/skybox", this.scene);
        skyboxMaterial.reflectionTexture.coordinatesMode = BABYLON.Texture.SKYBOX_MODE;
        skyboxMaterial.diffuseColor = new BABYLON.Color3(0, 0, 0);
        skyboxMaterial.specularColor = new BABYLON.Color3(0, 0, 0);
        skyboxMaterial.disableLighting = true;
        skybox.material = skyboxMaterial;
        
        this.scene.freezeActiveMeshes();
        
    }
    
    
    //render the scene using the appearance data
    render(appearancedata){
        
        //the list of objects passed in to be rendered
        let objectspassedtorender = [];
        
        
        
        //for each object in the appearance data
        for (let objectdata of appearancedata.objects){
            
            
            //get the name of the object
            let objectname = objectdata.name;
            
            //get the mesh if it exists
            let objectmesh = this.scene.getMeshByName(objectname);
            
            
            //if the mesh doesnt exist, create it
            if (objectmesh == null){
                
                //the type of mesh it is
                let cubedata = objectdata.mesh.Cube;
                let cylinderdata = objectdata.mesh.Cylinder;
                let timerdata = objectdata.mesh.Timer;
                
                if (cubedata != null){
                    
                    let options = {
                        height : cubedata.dimensions[0],
                        width  : cubedata.dimensions[1],
                        depth  : cubedata.dimensions[2],
                    };
                    
                    objectmesh = BABYLON.MeshBuilder.CreateBox(objectdata.name, options, this.scene);
                    objectmesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
                    
                    //if this has a mesh
                    if (cubedata.texture != null){
                        objectmesh.material.ambientTexture = new BABYLON.Texture(cubedata.texture, this.scene);
                    }
                    
                }
                else if (cylinderdata != null){
                    
                    let options = {
                        height : cylinderdata.dimensions[0],
                        diameter  : cylinderdata.dimensions[1],
                    };
                    
                    objectmesh = BABYLON.MeshBuilder.CreateCylinder(objectdata.name, options, this.scene);
                    objectmesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
                    
                    //if this has a mesh
                    if (cylinderdata.texture != null){
                    
                        objectmesh.material.ambientTexture = new BABYLON.Texture(cylinderdata.texture, this.scene);
                    }

                }
                else if (timerdata != null){
                    
                    let options = {
                        height : 1,
                        width  : 1,
                        depth  : 1,
                    };
                    
                    objectmesh = BABYLON.MeshBuilder.CreateBox(objectdata.name, options, this.scene);
                    objectmesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
                    
                }
                else{
                    console.log("THIS CARD DOESNT HAVE A MESH");
                }
                
            }
            
            
            
            
            //set its position and rotation values
            /*
            objectmesh.position.x = objectdata.position[0];
            objectmesh.position.y = objectdata.position[1];
            objectmesh.position.z = objectdata.position[2];
            */
            
            objectmesh.position.x = (objectmesh.position.x * 0.5) + (objectdata.position[0] * 0.5);
            objectmesh.position.y = (objectmesh.position.y * 0.5) + (objectdata.position[1] * 0.5);
            objectmesh.position.z = (objectmesh.position.z * 0.5) + (objectdata.position[2] * 0.5);
            
            
            objectmesh.rotation.x = objectdata.rotation[0];
            objectmesh.rotation.y = objectdata.rotation[1];
            objectmesh.rotation.z = objectdata.rotation[2];
            
            
            objectmesh.material.diffuseColor = new BABYLON.Color3( objectdata.colour[0] / 255, objectdata.colour[1] / 255, objectdata.colour[2] /255);
            
            
            
            let timerdata = objectdata.mesh.Timer;
            
            //if its a timer, scale it according to the time left
            if (timerdata != null){
                
                objectmesh.position.x = objectdata.position[0];
                objectmesh.position.y = objectdata.position[1];
                objectmesh.position.z = objectdata.position[2];
                
                objectmesh.scaling = new BABYLON.Vector3(1, timerdata.ticksleft / 10, 1);
                objectmesh.position.y = objectmesh.position.y + timerdata.ticksleft / 20 ;
                
            }
            
            
            objectspassedtorender.push(objectname);
        }
        
        
        
        //and each object that wasn't passed in for this tick, remove it from the list of meshes
        //if its name also isnt "plane"
        for (let mesh of this.scene.meshes) {
            
            //if the objects passed to render includes the current mesh
            if (objectspassedtorender.includes(mesh.name)) {
                //do nothing            
            }
            else{
                
                if (mesh.name == "plane" || mesh.name == "myMaterial" || mesh.name == "skybox"){
                }
                else{    
                    console.log("im disposing of", mesh.name);
                    mesh.dispose();
                }
                
            }
        }
        
        
        //FOR SOME REASON
        //WHICH THE KNOWLEDGE OF HAS BEEN GIFTED ONTO ME PURELY BY LUCK
        //I HAVE FOUND THIS TO BE THE SOLUTION TO THE FACT THAT NEW OBJECTS ADDED TO THE SCENE
        //WOULD BE ADDED AND THEIR MESHES AND SHIT ADDED
        //BUT WOULDNT ACTUALLY APPEAR
        //WHEN I DISPOSE OF A MESH, IT FOR WHATEVER REASON UPDATES THE SCENE SO THAT IT BECOMES VISIBLE
        //AT LEAST NOW I HAVE ENOUGH IN THE JS SIDE THAT I DONT HAVE TO DEAL WITH THIS
        //THIS IS WHY I HATE JAVASCRIPT
        //THE LIBRARIES ARE JUST WORSE
        //AND EVERY OBJECT HAS LIKE 200 PROPERTIES AND METHODS
        //cant stress how annoying and pointless things like this are that i seem to only encounter in js
        let objectmesh = BABYLON.MeshBuilder.CreateBox("benis", {}, this.scene);
        objectmesh.dispose();
        
        
        this.scene.render();
        
        
    }
}


function pad(n, width, z) {
    z = z || '0';
    n = n + '';
    return n.length >= width ? n : new Array(width - n.length + 1).join(z) + n;
}





//this class is called when the player creates a new game
class GameInterface{
    
    
    
    constructor(engine, socket){
        
        //create the "appearance" object for this game, giving it the scene of the engine
        this.gameappearance = new GameApperance(engine, this);
        
        this.socket = socket;
        
        //create the wasm game
        this.wasmgame = FullGame.new(1);
        
        
        //if an object is being dragged (if the camera movement is disabled)
        this.draggingobject = false;
        
        //what the position of the pointer is on the y=1.5 plane when i start dragging
        this.draggingstartingposition = null;
        
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
            
            //and send them to the server
            this.socket.send( this.wasmgame.pop_outgoing_socket_message() );
        }
        
    }
    
    
    
    //when a player clicks
    mouseup(){
        
        //reenable the cameras ability to move
        this.gameappearance.camera.inputs.attached["mousewheel"].wheelPrecision = 10;
        this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityX = 1000;
        this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityY = 1000;
        
        
        //not dragging any object after the mouse is lifted
        this.draggingobject = false;
        
        //tell the wasm that its mouse up
        //so it can send the flick missions if any piece is in the middle of being flicked
        this.wasmgame.mouse_up();
        
    }
    
    
    
    //when the mouse is moved
    mousemove(){
        
        //if a piece is currently being dragged, send that information to the wasmgame
        if (this.draggingobject){
            
            
            let selectedobjectname = this.wasmgame.get_selected_object_name();
            
            var objectunder = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
                
                return mesh.name != "plane" && mesh.name != "dragindicator" && mesh.name != selectedobjectname;  // the plane and drag indicator will not be pickable
                
            });
            
            
            
            //set the position of the cursor on the plane
            var pickResult = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
                return mesh.name == "plane";  // the plane will be the only pickable thing
            });
            
            let draggingcurposition = [pickResult.pickedPoint.x, pickResult.pickedPoint.z];
            
            
            let distancedraggedx = draggingcurposition[0] - this.draggingstartingposition[0];
            let distancedraggedz = draggingcurposition[1] - this.draggingstartingposition[1];
            
            
            if (objectunder.pickedMesh ==  null){
                
                this.wasmgame.drag_selected_object(distancedraggedx, distancedraggedz, "");
                
            }
            else{
                
                this.wasmgame.drag_selected_object(distancedraggedx, distancedraggedz, objectunder.pickedMesh.name);
                
            }
            
            
        }
        
        
        
    }
    
    
    //when the mouse goes down
    mousedown(){
        
        var pickResult = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
            
            return mesh.name != "plane" && mesh.name != "dragindicator";  // the plane and drag indicator will not be pickable
        });
        
        
        console.log(pickResult.pickedMesh.name);
        
        
        //if a mesh has been clicked
        let clickedobject = pickResult.pickedMesh;
        
        
        
        
        
        //if an object was clicked on
        if (clickedobject != null) {
            
            let clickedobjectname = clickedobject.name;
            
            //if the clicked object has a name and it isnt "plane"
            if (clickedobjectname != null){
                
                
                //if the object is already selected, and is flickable
                if (this.wasmgame.is_object_selected(clickedobjectname)){
                    
                    
                    //disable panning rotating, all camera movement basically
                    //and remporarily
                    //dont disable scrolling, it wont affect anything the player doesnt want affected when dragging
                    //this.gameappearance.camera.inputs.attached["mousewheel"].wheelPrecision = 100000;
                    this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityX = 1000000;
                    this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityY = 1000000;
                    
                    this.draggingobject = true;
                    
                    
                    //set the position of the cursor on the plane
                    var pickResult = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
                        return mesh.name == "plane";  // the plane will be the only pickable thing
                    });
                    
                    this.draggingstartingposition = [pickResult.pickedPoint.x, pickResult.pickedPoint.z];
                    
                    
                }
                //if its not already the selected object, or is not flickable
                else{
                    this.wasmgame.click_object( clickedobjectname);
                }
            }
            //if the clicked object doesnt have a name, set the selected mesh to none
            else{
                this.wasmgame.click_object("");
            }
        }
        //if it wasnt, clear the selected object
        else{
            this.wasmgame.click_object("");
        }
        
        
        
    }
    
    
    
}