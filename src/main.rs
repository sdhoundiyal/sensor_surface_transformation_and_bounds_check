pub struct SensorSurface{ // a structure to define a sensor surface
	id: u32, // unique id for sensor surface
	affine32: [[f32; 4]; 4], // affine transform to convert global cordinates to local cordinates
    affine23: [[f32; 4]; 4], // affine transform to convert local cordinates to global cordinates
    bounds: Vec<f32>, // bounds of the sensor in local cordinate system
    shape: u32, // identifier for the shape of the sensor
}
#[macro_use]
extern crate text_io;


fn main() {
	let mut sensors: Vec<SensorSurface> = Vec::new(); // a vector of sensor 
	println!("Please define a Sensor-surface to begin");
	sensors.push(define_new_sensor_surface()); // user defines a sensor-surface to begin with 
	loop {
		let mut flag = false;
		println!("1. Define a new sensor-surface");// menu for the user
		println!("2. Input a point in the Global System, convert to a local system, and check bounds");
		println!("3. Input a point in a local system, check bounds and convert to Global System");
		println!("4. Exit");
		println!("Enter your choice");
		let choice = read!();
		match choice {
            1 => sensors.push(define_new_sensor_surface()), // define new sensor-surface
            2 => { // enter global cordinates and process them, convert to a local system, check bounds
            	let point = accept_global_point(); // user enters global cordinates of a point
            	println!("Select a sensor surface, enter id"); // user selects  a sensor-surface
            	let sensor_selected:u32 = read!();
            	let mut sense = &sensors[0]; 
            	for i in &sensors { // search for the selected sensor-surface
            		if sensor_selected == i.id {
            			flag = true;
            			sense = i;
            			break;
            		}
            	}
            	if flag { // sensor-surface is found
            		process_global_point(sense, &point);
            	}
            	else { // sensor-surface not found
            		println!("Surface not found, please enter valid details");
            	}
            }
            3 => { // enter local cordinates and process them, check bounds, convert to global system
            	let point = accept_local_point(); // user enters local cordinates of a point
            	println!("Select a sensor surface, enter id"); // user selects the sensor-surface
            	let sensor_selected:u32 = read!();
            	let mut sense = &sensors[0]; 
            	for i in &sensors {
            		if sensor_selected == i.id {
            			flag = true;
            			sense = i;
            			break;
            		}
            	}
            	if flag { // sensor-surface is found
            		process_local_point(sense, &point);
            	}
            	else { // sensor surface not found
            		println!("Surface not found, please enter valid details");
            	}
            }
            4 => break, // user decides to exit
            _ => println!("Choice does not exist, select a valid option."), // invalid choice 
        }
	}

}


fn accept_origin() -> Vec<f32> { //User enters origin for the local cordinate system
	println!("Please specify the origin for the sensor");
	
	println!("Enter X co-ordinate:");
	let x: f32 = read!();
    
    println!("Enter Y co-ordinate:");
	let y: f32 = read!();
    
    println!("Enter Z co-ordinate:");
	let z: f32 = read!();
    
    vec![x,y,z]
}

fn accept_basis_vector() -> Vec<f32> { //User specifies direction of local axis in terms of angles with the global axes

	println!("with the X-axis:");
	let mut x: f32 = read!();
	x=x.cos();

	println!("with the Y-axis:");
	let mut y: f32 = read!();
	y=y.cos();

	println!("with the Z-axis:");
	let  mut z: f32 = read!();
    z=z.cos();
	
	vec![x,y,z]
}



fn accept_local_bounds( sensor_type: u32) -> Vec<f32> { //User specifies bounds in local coordinate system 
    println!("Please specify the bounds for the sensor");
    match sensor_type { // bounds are accepted based on shaped of sensor
        0 => accept_rectangular_bounds(),
        _ => vec![0.0,0.0],
    }
}

fn accept_rectangular_bounds() -> Vec<f32> { //User specifies bounds of a rectangular sensor
    println!("Enter bound along X-axis");
    let x: f32 = read!();
    println!("Enter bound along Y-axis");
    let y: f32 = read!();

    vec![x,y]
}


fn define_local_surface(x_axis: &[f32], y_axis: &[f32], origin: &[f32]) -> Vec<f32> { // determine the Equation of plane containing the sensor is determined based on the Origin and the two axes vectors
    let a=&x_axis[1]*&y_axis[2]-&x_axis[2]*&y_axis[1];
    let b=&x_axis[2]*&y_axis[0]-&x_axis[0]*&y_axis[2];
    let c=&x_axis[0]*&y_axis[1]-&x_axis[1]*&y_axis[0];
    let d=a*&origin[0]+b*&origin[1]+c*&origin[2];

    vec![a,b,c,d]
}

fn define_32_rotation_matrix(x_axis: &[f32],y_axis: &[f32]) -> [[f32; 4]; 4] { // define Global to Local rotation matrix
	let mut rotate = [[0f32; 4]; 4];
	rotate[0][0] = x_axis[0];
	rotate[0][1] = x_axis[1];
	rotate[0][2] = x_axis[2];

	rotate[1][0] = y_axis[0];
	rotate[1][1] = y_axis[1];
	rotate[1][2] = y_axis[2];
	
    rotate[2][2] = 1.0;
    rotate[3][3] = 1.0;

    rotate
}

fn define_32_translation_matrix(origin: &[f32]) -> [[f32; 4]; 4] { // define Global to Local translation matrix
	let mut translate = [[0f32; 4]; 4];
    translate[0][0] = 1.0;
    translate[1][1] = 1.0;
    translate[2][2] = 1.0;
    translate[3][3] = 1.0;

    translate[3][0] = -1.0*origin[0];
    translate[3][1] = -1.0*origin[0];
    translate[3][2] = -1.0*origin[0];

    translate
}

fn define_23_rotation_matrix(x_axis: &[f32],y_axis: &[f32]) -> [[f32; 4]; 4] { //define Local to Global rotation matrix
	let mut rotate = [[0f32; 4]; 4];
	rotate[0][0] = x_axis[0];
	rotate[1][1] = y_axis[1];
	rotate[2][2] = 1.0;
	rotate[3][3] = 1.0;

	rotate
}

fn define_23_zcordinate(plane: &[f32]) -> [[f32; 4]; 4] { // determine z coordinate from  equation of plane
    let mut det_z = [[0f32; 4]; 4];
    det_z[0][0] = 1.0;
    det_z[1][1] = 1.0;
    det_z[2][0] = -1.0*plane[0]/plane[2];
    det_z[2][1] = -1.0*plane[1]/plane[2];
    det_z[2][2] = plane[3];
    det_z[3][3] = 1.0;

    det_z
}

fn define_23_translation_matrix(origin: &[f32]) -> [[f32; 4]; 4] { //define Local to Global translation matrix
	let mut translate = [[0f32; 4]; 4];
	translate[0][0] = 1.0;
	translate[1][1] = 1.0;
	translate[2][2] = 1.0;
	translate[3][3] = 1.0;
	translate[0][3] = origin[0];
	translate[1][3] = origin[1];
	translate[2][3] = origin[2];

	translate
}

pub fn define_new_sensor_surface() -> SensorSurface{ //User defines a new sensor surface, its Local cordinate system, and the Global-Local conversion matricies are calculated
	println!("Please follow the instructions to define a new sensor:");

	println!("Please specify a numeric id for the sensor-surface"); // user gives the sensor-surface a new id
	let surface_id: u32 = read!();
	
	let l_origin = accept_origin(); // user enters origin of local cordinate system in global cordinate system
	println!("Please Specify the angles the local X-axis makes with the global axes in radians:");
	let x_basis = accept_basis_vector(); // user defines x-axis of local cordinate system in terms of angles with global axis
	println!("Please Specify the angles the local Y-axis makes with the global axes in radians:");
	let y_basis = accept_basis_vector(); // user defines x-axis of global cordinate system in terms of angles with global axis
	
	let l_plane = define_local_surface(&x_basis,&y_basis,&l_origin); // equation of plance containing sensor-surface is calculated
	
	println!("Please specify shape of the sensor:");
	let shape_id:Vec<u32> = vec![0]; 
	let shape_name:Vec<&str> = vec!["Rectangular"];
	for i in 0..1{
		println!("{} - {}", shape_id[i], shape_name[i]);
	}
	let l_shape: u32 = read!(); // user specifies the shape of the sensor of available shapes
	let l_bounds = accept_local_bounds(l_shape); // user specifies bounds in local cordinate system
	
	let rotate32 = define_32_rotation_matrix(&x_basis,&y_basis); // calculate global to local rotation matrix
	let translate32 = define_32_translation_matrix(&l_origin); // calculate global to local translation matrix
    let l_affine32 = multiply_matricies(&rotate32,&translate32); // calculate global to local affine transform matrix as product of rotation and translation matrix

	let rotate23 = define_23_rotation_matrix(&x_basis,&y_basis); // calculate local to global rotation matrix
	let getz23 = define_23_zcordinate(&l_plane); // matrix which determines global z cordiante using equation of plane
	let translate23 = define_23_translation_matrix(&l_origin); //calculate local to global translation matrix
	let temp = multiply_matricies(&getz23,&rotate23); 
	let l_affine23 = multiply_matricies(&translate23,&temp); // calculate local to global affine transform matrix as product of all three matricies
	println!("3 to 2");
	for i in 0..4 {
		for j in 0..4 {
			println!("{}", l_affine32[i][j]);
		}
	}
	println!("2 to 3");
	for i in 0..4 {
		for j in 0..4 {
			println!("{}", l_affine23[i][j]);
		}
	}
	SensorSurface { id: surface_id, affine32: l_affine32, affine23: l_affine23, bounds: l_bounds, shape: l_shape} // return sensor-surface information as instance of structure
}

fn multiply_matricies(first: &[[f32; 4]; 4], second: &[[f32; 4]; 4])-> [[f32; 4]; 4] { //performs matrix multiplication for 2 4X4 matricies
	let mut product = [[0f32; 4]; 4];
	for i in 0..4 {
		for j in 0..4 {
			for k in 0..4 {
				product[i][j]=product[i][j]+first[i][k]*second[k][j];
			}
		}
	}
	product
}

fn multiply_matrix_vector(matrix: &[[f32; 4]; 4], vector: &[f32]) -> Vec<f32> { //multiply a vector with a matrix 
	let mut output = vec![0.0,0.0,0.0,0.0];
	for i in 0..4 {
		for j in 0..4 {
			output[i] = &output[i] + &matrix[i][j]*&vector[j];
		}
	}
	output
}


pub fn accept_local_point() -> Vec<f32> { //Accept local cordinates
	println!("Please enter cordiantes of the point");
	println!("X-cordinate");
	let x:f32 = read!();
	println!("Y-cordinate");
	let y:f32 = read!();
	vec![x,y,1.0,1.0]
}

pub fn process_local_point(sensor_surface: &SensorSurface, point: &[f32]){ //check bounds and convert local to global cordinates
	if test_bounds(&sensor_surface.bounds, point, sensor_surface.shape) {
		println!("Point is within bounds.");
	}
	else {
		println!("Point is outside bounds.")
	}
	let global_point = convert_to_global(&sensor_surface.affine23,point);
	println!("The global cordinates of the point are ({},{},{})",global_point[0],global_point[1],global_point[2]); 
}

fn convert_to_global(affine:  &[[f32; 4]; 4], point: &[f32]) -> Vec<f32> { //convert local to global cordinate system
	let global = multiply_matrix_vector(&affine,point);
	vec![global[0], global[1], global[2]]
}

fn test_bounds(bound: &[f32], point: &[f32], shape: u32)-> bool { // check if a point in the local cordinae system is within bounds
	let mut flag: bool = false;
	match shape {
		0 => flag = bound[0] >= point[0].abs() && bound[1] >= point[1].abs(),
		_ => flag = false,
	}
	flag
}

pub fn  accept_global_point() -> Vec<f32> { //Accept global cordinates
	println!("Please enter cordinates of the point");
	println!("X-cordinate");
	let x:f32 = read!();
	println!("Y-cordinate");
	let y:f32 = read!();
	println!("Z-cordinate");
	let z:f32 = read!();
	vec![x,y,z,1.0]
}

pub fn process_global_point(sensor_surface: &SensorSurface, point: &[f32]) { //convert global to local cordinates and check bounds
	let local_point = convert_to_local(&sensor_surface.affine32,point);
	println!("The local cordinate of the point are ({},{})",local_point[0],local_point[1]);
	if  test_bounds(&sensor_surface.bounds, point, sensor_surface.shape) {
		println!("Point is within bounds.");
	}
	else {
		println!("Point is outside bounds.")
	}
}

fn convert_to_local(affine:  &[[f32; 4]; 4], point: &[f32]) -> Vec<f32> { //convert local to global cordinate system
	let local = multiply_matrix_vector(&affine,point);
	vec![local[0], local[1]]
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_within_local_bounds() {	
        assert_eq!(test_bounds(&vec![1.93,3.45], &vec![1.94, 3.45, 1.0, 1.0], 0), false);
    }

    #[test]
    fn test_outside_local_bounds() {	
        assert_eq!(test_bounds(&vec![1.93,3.45], &vec![1.84, 3.43, 1.0, 1.0], 0), true);
    }

    #[test]
    fn test_global_to_local_transform() {
    	let aff:[[f32; 4]; 4] =[[0.2481754,-0.99999875,-0.5389615,0.0],[-0.99999875,-0.93533456,0.12050225,0.0],[0.0,0.0,1.0,0.0],[-3.0,-3.0,-3.0,1.0]];
        let out = convert_to_local(&aff,&vec![2.17,3.19,4.17,1.0]);
        let tes = vec![-4.898925,-4.6512184];
        let mut flag: bool = true;
        for i in 0..2 {
        	if &out[i]!=&tes[i] {
        		flag = false;
        	}
        }
        assert_eq!(flag,true);
    }
    #[test]
    fn test_local_to_global_transform() {
    	let aff:[[f32; 4]; 4] =[[0.2481754,-0.99999875,-0.5389615,3.4113173],[-0.99999875,-0.93533456,0.12050225,4.38865637],[0.0,0.0,1.0,-4.0],[0.0,0.0,0.0,1.0]];
        let out = convert_to_global(&aff,&vec![2.17,3.19,4.17,1.0]);
        let tes = vec![-1.48760759,-0.26256378];
        let mut flag: bool = true;
        for i in 0..2 {
        	if &out[i]!=&tes[i] {
        		flag = false;
        	}
        }
        assert_eq!(flag,true);
    }



    

}