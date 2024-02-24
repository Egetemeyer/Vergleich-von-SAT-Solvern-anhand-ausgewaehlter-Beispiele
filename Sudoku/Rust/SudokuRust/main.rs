fn step5(first_index_i_loop: usize, last_index_i_loop: usize, first_index_j_loop: usize, last_index_j_loop: usize,
            array: &[[[kissat::Var; 9]; 9]; 9], solver: &mut kissat::Solver){
                
    for k in 0..array.len(){
        //Erstes Element in der Box
        let mut temp = array[first_index_i_loop][first_index_j_loop][k];
        for i in first_index_i_loop..last_index_i_loop{
            for j in first_index_j_loop..last_index_j_loop{
                temp = solver.or(temp, array[i][j][k]);
                //println!("array[{i}][{j}][{k}]");
            }
        }
        //println!("ADD");
        solver.add1(temp);
    }
}

fn main() {

    let mut solver = kissat::Solver::new();

    let init_variable = solver.var();

    //Idee Array beim definieren schon mit allen Variablen initialisieren
    //mit 3-Dimensionalem Array
    let mut array: [[[kissat::Var; 9]; 9]; 9] = [[[init_variable; 9]; 9]; 9];

    

    //let mut counter = 0;

    //Initialisieren der Variablen

    for i in 0..array.len(){
        for j in 0..array.len(){
            for k in 0..array.len(){

                //counter += 1;
                let variable = solver.var();

                array[i][j][k] = variable;
                //println!("array[{i}][{j}][{k}]");
            }
        }
    }
    //println!("Anzahl Variablen: {counter}");



    for j in 0..array.len(){

        for x in 0..array.len(){

            //1. Schritt    -- Mindestens ein Value pro Feld
            let mut term = solver.var();
            for i in 0..(array.len()-1){
                //(Z_111 OR Z_112 OR Z_113 OR Z_114 OR Z_115 
                //      OR Z_116 OR Z_117 OR Z_118 OR Z_119)
                if i == 0 {
                    term = solver.or(array[j][x][i], array[j][x][i+1]);
                }
                //Wenn i > 0, also term schon mit dem ersten OR erstellt wurde
                else{
                    //Lange OR-Verknüpfung erstellen
                    term = solver.or(term, array[j][x][i+1]);
                }

                //True im letzen Durchlauf
                if i == (array.len()-2){
                    solver.add1(term);
                }

            }
        }

    }
    


    

    //2. Schritt    -- Maximal ein Value pro Feld
    //...

    //1. For-Schleife vergleicht von index i bis zum letzten index
    //Danach wird vom index i+1 bis zum letzen index verglichen
    //Das ganze muss n-1 mal ausgeführt werden


    for x in 0..array.len(){
        for y in 0..array.len(){

            //Wird nur 8-mal durchlaufen, da der letzte index nicht nochmal 
            //explizit mit einem andern Wert verglichen werden müsste
            for i in 0..(array.len()-1){
                //println!("Neue Iteration");
                //Beginnt bei i+1, da man j nicht mit der selben Variable vergleichen muss
                for j in (i+1)..array.len(){
                    //(!Z_111 OR !Z_112) AND (!Z_111 OR !Z113) ...
                    //...
                    //AND (!Z_118 OR !Z_119)
                    //println!("i: {i} - j: {j}");
                    let term = solver.or(!array[x][y][i], !array[x][y][j]);
                    solver.add1(term);
                }

            }

        }

    }




    //3. Schritt -- Jede Zeile hat alle Zahlen
    
    /*Bsp. 1. Zeile
        Es geht immer nach rechts in der Zeile,
        und da bereits nur maximal 1 Wert pro Feld aktiv sein kann
    reicht eine OR-Verbindung*/

    //(Z_111 OR Z_121 OR Z_131 OR Z_141 OR ... OR Z_191)
    //Im Array ist die 1. Zeile mit dem Index 0 versehen

    //j = Zeile
    for j in 0..array.len(){
        //k = Value
        for k in 0..array.len(){
            let mut temp = solver.var();
            for i in 0..(array.len()-1){
        
                if i == 0 {
                    temp = solver.or(array[j][i][k], array[j][i+1][k]);
                }
                else{
                    temp = solver.or(temp, array[j][i+1][k]);
                }
                if i == (array.len()-2){
                    solver.add1(temp);
                }        
            }
        }
    }
    
    //4. Schritt: Jede Spalte hat alle Zahlen

    for j in 0..array.len(){
        for k in 0..array.len(){
            let mut temp = solver.var();
            for i in 0..(array.len()-1){
                
                if i == 0 {
                    temp = solver.or(array[i][j][k], array[i+1][j][k]);
                }
                else{
                    temp = solver.or(temp, array[i+1][j][k]);
                }
                if i == (array.len()-2){
                    solver.add1(temp);
                }        
            }
        }
    }


    //5. Schritt: Jeder 3x3 Block hat alle Zahlen
    step5(0,3,0,3, &array, &mut solver);
    step5(0,3,3,6, &array, &mut solver);
    step5(0,3,6,9, &array, &mut solver);

    step5(3,6,0,3, &array, &mut solver);
    step5(3,6,3,6, &array, &mut solver);
    step5(3,6,6,9, &array, &mut solver);

    step5(6,9,0,3, &array, &mut solver);
    step5(6,9,3,6, &array, &mut solver);
    step5(6,9,6,9, &array, &mut solver);
    


    //6. Schritt: Selbst erzeugtes Feld eingeben    #Feld aus Bildquelle nehmen
    /*
    {
        //1. Zeile
        solver.add1(array[0][1][5]);
        solver.add1(array[0][2][7]);
        solver.add1(array[0][4][1]);
        solver.add1(array[0][5][3]);
        solver.add1(array[0][6][8]);

        //2. Zeile
        solver.add1(array[1][0][2]);
        solver.add1(array[1][1][3]);
        solver.add1(array[1][3][4]);
        solver.add1(array[1][4][0]);
        solver.add1(array[1][7][7]);
        solver.add1(array[1][8][6]);

        //3. Zeile
        solver.add1(array[2][0][0]);
        solver.add1(array[2][2][6]);
        solver.add1(array[2][3][2]);
        solver.add1(array[2][4][7]);
        solver.add1(array[2][7][4]);

        //4. Zeile
        solver.add1(array[3][1][7]);
        solver.add1(array[3][2][4]);
        solver.add1(array[3][4][2]);
        solver.add1(array[3][5][0]);
        solver.add1(array[3][6][3]);
        solver.add1(array[3][7][6]);

        //5. Zeile
        solver.add1(array[4][0][1]);
        solver.add1(array[4][1][0]);
        solver.add1(array[4][2][8]);
        solver.add1(array[4][5][6]);

        //6. Zeile
        solver.add1(array[5][2][3]);
        solver.add1(array[5][8][1]);

        //7. Zeile
        solver.add1(array[6][0][3]);
        solver.add1(array[6][1][6]);
        solver.add1(array[6][2][2]);
        solver.add1(array[6][3][5]);
        solver.add1(array[6][7][8]);
        solver.add1(array[6][8][0]);

        //8. Zeile
        solver.add1(array[7][7][3]);

        //9. Zeile
        solver.add1(array[8][0][7]);
        solver.add1(array[8][5][2]);
        solver.add1(array[8][8][5]);
    }
    */

    //Anderes Feld -- Aus dem Maple-Beispiel
    {
        //1. Zeile
        solver.add1(array[0][0][7]);

        //2. Zeile
        solver.add1(array[1][2][2]);
        solver.add1(array[1][3][5]);

        //3. Zeile
        solver.add1(array[2][1][6]);
        solver.add1(array[2][4][8]);
        solver.add1(array[2][6][1]);

        //4. Zeile
        solver.add1(array[3][1][4]);
        solver.add1(array[3][5][6]);

        //5. Zeile
        solver.add1(array[4][4][3]);
        solver.add1(array[4][5][4]);
        solver.add1(array[4][6][6]);

        //6. Zeile
        solver.add1(array[5][3][0]);
        solver.add1(array[5][7][2]);

        //7. Zeile
        solver.add1(array[6][2][0]);
        solver.add1(array[6][7][5]);
        solver.add1(array[6][8][7]);

        //8. Zeile
        solver.add1(array[7][2][7]);
        solver.add1(array[7][3][4]);
        solver.add1(array[7][7][0]);

        //9. Zeile
        solver.add1(array[8][1][8]);
        solver.add1(array[8][6][3]);
    }


    //Ausgangsspielfeld ausgeben

    

    


    match solver.sat() {
        Some(solution) => {

        
            /*
            for i in 0..array.len(){
                for j in 0..array.len(){
                    println!("Neues Feld:");
                    for k in 0..array.len(){
                        println!("array[{i}][{j}][{k}] = {:?}", solution.get(array[i][j][k]).unwrap());
                    }
                }
            }
            */
            

            /*
            for i in 0..array.len(){
                println!("array[0][0][{i}] = {:?}", solution.get(array[0][0][i]).unwrap());
            }
            */

            println!("\n\n");
            println!("Sudoku-Feld mit Informatiker Werten:");
            //Sudoku-Feld Informatik ausgeben
        
            //Der Name der Variablen kommt in das Feld, für das der Wert true ist
            let mut fieldcounter = 0;
            let mut rowcounter = 0;
            //i = Y-Richtung im Sudoku-Feld
            for i in 0..array.len(){
                rowcounter += 1;
                //j = X-Richtung im Sudoku-Feld
                for j in 0..array.len(){

                    //Variable mit Wert true suchen
                    for k in 0..array.len(){
                        if solution.get(array[i][j][k]) == Some(true){
                            fieldcounter += 1;
                            //Ziffer ist nur wichtig
                            //print!("[{i}][{j}][{k}] ");
                            print!("{k} ");
                            if fieldcounter % 3 == 0{
                                if fieldcounter % 9 != 0{
                                    print!("| ");
                                }
                                
                            }
                            if fieldcounter % 9 == 0{
                                println!();
                            }
                            
                        }
                    }

                }
                if rowcounter % 3 == 0 {
                    println!();
                }
            }
        
            println!("Bitte beachten, die Ziffern gehen von 0 bis 8!!!");

            println!("Anzahl an Feldern: {fieldcounter}\n\n");


            //Sudoku-Feld Laie ausgeben
            println!("Sudoku-Feld mit 'normalen' Werten:");
            //Der Name der Variablen kommt in das Feld, für das der Wert true ist
            let mut fieldcounter = 0;
            let mut rowcounter = 0;
            //i = Y-Richtung im Sudoku-Feld
            for i in 0..array.len(){
                rowcounter += 1;
                //j = X-Richtung im Sudoku-Feld
                for j in 0..array.len(){

                    //Variable mit Wert true suchen
                    for k in 0..array.len(){
                        if solution.get(array[i][j][k]) == Some(true){
                            fieldcounter += 1;
                            //Ziffer ist nur wichtig
                            //print!("[{i}][{j}][{k}] ");
                            print!("{} ", (k+1));
                            if fieldcounter % 3 == 0{
                                if fieldcounter % 9 != 0{
                                    print!("| ");
                                }
                                
                            }
                            if fieldcounter % 9 == 0{
                                println!();
                            }
                            
                        }
                    }

                }
                if rowcounter % 3 == 0 {
                    println!();
                }
            }




        }
        None => println!("UNSAT"),
    }

}
