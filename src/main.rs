#![feature(new_uninit)]


mod wave_equation;

fn array_test(array: &[f32], array2: &mut[f32])
{
    array2[10] = array[10];
}

fn grid_test<const M:usize, const N:usize>(array: &[[f32;M];N], mut array2: &[[f32;M];N],)
{

}

fn main() {

    const N: usize = 100;
    const M: usize = 200;

    let mut grid1 = unsafe {Box::<[[f32; M]; N]>::new_zeroed().assume_init()};
    let mut grid2 = unsafe {Box::<[[f32; M]; N]>::new_zeroed().assume_init()};
    let mut grid3 = unsafe {Box::<[[f32; M]; N]>::new_zeroed().assume_init()};

    const H: f32 = 1.0;
    const DELTA_T: f32 = 0.0001;



    let mut previous =  & mut *grid1;
    let mut current = & mut *grid2;
    let mut next = & mut *grid3;

    for i in 0..1000 
    {
        wave_equation::wave_equation_step(&previous, &current, & mut next, DELTA_T, H);

        let tmp = previous;
        previous = current;
        current = next;
        next = tmp;
    }



    // let mut arr1: [f32; 500] = [0.0; 500];
    // let mut arr2: [f32; 500] = [0.0; 500];

    // array_test(&arr1, &mut arr2);

    // let mut grid1_s: [[f32; 500]; 500] = [[0.0; 500]; 500];
    // let mut grid2_s: [[f32; 500]; 500] = [[0.0; 500]; 500];

    // grid_test(&grid1_s, &grid2_s);


    println!("Hello, world!"); 
}
