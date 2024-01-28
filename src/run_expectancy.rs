/// run_expectancy tables are built using the pitch by pitch data. There are 2 potnetial algorithms to generate the tables:
/// 
/// ## Method 1: Use averages
///     1) For each venue, season and sport_id, calculate the average number of runs scored for after reaching a specific 288 state. <br/>
///     2) Then we make sure that for items that have sample size < 30, we take the average of the two neighbours. Neighbours will be classified
///     statically.
/// 
/// ## Methos 2: Simulation
///     1) For each venue, season and sport_id, calculate the probability of each high-level event type (ball, strike, in-play)
///     2) The in-play probability (specifically single/double/triple) is dependent on the ball-strike-out state.
///     3) Simulate seasons based on the above probabilities.
 

/// RE288: The run expectancy for all 288 possible base, out, ball, strike states
#[derive(Debug)]
pub struct RE288 {
    pub outs: u8,
    pub balls: u8,
    pub strikes: u8,
    pub base_value: u8,
    pub run_expectancy: f32,
}

///Default RE288 Value based on Tom Tango's 2018 RE288 numbers. 
pub const RE288_DEFAULT: [RE288; 288] = [
    RE288 {balls: 3, strikes:  0, base_value: 7, outs: 0, run_expectancy: 2.912},
    RE288 {balls: 3, strikes:  2, base_value: 7, outs: 0, run_expectancy: 2.742},
    RE288 {balls: 3, strikes:  1, base_value: 7, outs: 0, run_expectancy: 2.696},
    RE288 {balls: 2, strikes:  0, base_value: 7, outs: 0, run_expectancy: 2.549},
    RE288 {balls: 1, strikes:  0, base_value: 7, outs: 0, run_expectancy: 2.51},
    RE288 {balls: 2, strikes:  1, base_value: 7, outs: 0, run_expectancy: 2.505},
    RE288 {balls: 0, strikes:  0, base_value: 7, outs: 0, run_expectancy: 2.376},
    RE288 {balls: 1, strikes:  1, base_value: 7, outs: 0, run_expectancy: 2.352},
    RE288 {balls: 2, strikes:  2, base_value: 7, outs: 0, run_expectancy: 2.345},
    RE288 {balls: 3, strikes:  0, base_value: 6, outs: 0, run_expectancy: 2.285},
    RE288 {balls: 0, strikes:  1, base_value: 7, outs: 0, run_expectancy: 2.247},
    RE288 {balls: 3, strikes:  1, base_value: 6, outs: 0, run_expectancy: 2.217},
    RE288 {balls: 3, strikes:  0, base_value: 7, outs: 1, run_expectancy: 2.197},
    RE288 {balls: 1, strikes:  2, base_value: 7, outs: 0, run_expectancy: 2.165},
    RE288 {balls: 0, strikes:  2, base_value: 7, outs: 0, run_expectancy: 2.14},
    RE288 {balls: 3, strikes:  0, base_value: 5, outs: 0, run_expectancy: 2.097},
    RE288 {balls: 2, strikes:  0, base_value: 6, outs: 0, run_expectancy: 2.09},
    RE288 {balls: 3, strikes:  1, base_value: 5, outs: 0, run_expectancy: 2.039},
    RE288 {balls: 3, strikes:  0, base_value: 3, outs: 0, run_expectancy: 2.017},
    RE288 {balls: 1, strikes:  0, base_value: 6, outs: 0, run_expectancy: 2.014},
    RE288 {balls: 2, strikes:  1, base_value: 6, outs: 0, run_expectancy: 2.002},
    RE288 {balls: 3, strikes:  2, base_value: 6, outs: 0, run_expectancy: 2.001},
    RE288 {balls: 3, strikes:  1, base_value: 7, outs: 1, run_expectancy: 1.962},
    RE288 {balls: 0, strikes:  0, base_value: 6, outs: 0, run_expectancy: 1.96},
    RE288 {balls: 1, strikes:  1, base_value: 6, outs: 0, run_expectancy: 1.958},
    RE288 {balls: 2, strikes:  0, base_value: 5, outs: 0, run_expectancy: 1.928},
    RE288 {balls: 0, strikes:  1, base_value: 6, outs: 0, run_expectancy: 1.884},
    RE288 {balls: 2, strikes:  0, base_value: 7, outs: 1, run_expectancy: 1.847},
    RE288 {balls: 1, strikes:  0, base_value: 5, outs: 0, run_expectancy: 1.843},
    RE288 {balls: 3, strikes:  1, base_value: 3, outs: 0, run_expectancy: 1.833},
    RE288 {balls: 2, strikes:  1, base_value: 5, outs: 0, run_expectancy: 1.83},
    RE288 {balls: 3, strikes:  2, base_value: 5, outs: 0, run_expectancy: 1.827},
    RE288 {balls: 2, strikes:  2, base_value: 6, outs: 0, run_expectancy: 1.82},
    RE288 {balls: 0, strikes:  0, base_value: 5, outs: 0, run_expectancy: 1.787},
    RE288 {balls: 1, strikes:  2, base_value: 6, outs: 0, run_expectancy: 1.758},
    RE288 {balls: 1, strikes:  1, base_value: 5, outs: 0, run_expectancy: 1.732},
    RE288 {balls: 2, strikes:  0, base_value: 3, outs: 0, run_expectancy: 1.725},
    RE288 {balls: 3, strikes:  2, base_value: 7, outs: 1, run_expectancy: 1.721},
    RE288 {balls: 0, strikes:  2, base_value: 6, outs: 0, run_expectancy: 1.708},
    RE288 {balls: 1, strikes:  0, base_value: 7, outs: 1, run_expectancy: 1.699},
    RE288 {balls: 0, strikes:  1, base_value: 5, outs: 0, run_expectancy: 1.69},
    RE288 {balls: 2, strikes:  2, base_value: 5, outs: 0, run_expectancy: 1.673},
    RE288 {balls: 2, strikes:  1, base_value: 7, outs: 1, run_expectancy: 1.664},
    RE288 {balls: 3, strikes:  2, base_value: 3, outs: 0, run_expectancy: 1.634},
    RE288 {balls: 3, strikes:  0, base_value: 4, outs: 0, run_expectancy: 1.608},
    RE288 {balls: 1, strikes:  2, base_value: 5, outs: 0, run_expectancy: 1.595},
    RE288 {balls: 0, strikes:  2, base_value: 5, outs: 0, run_expectancy: 1.592},
    RE288 {balls: 0, strikes:  0, base_value: 7, outs: 1, run_expectancy: 1.578},
    RE288 {balls: 3, strikes:  0, base_value: 6, outs: 1, run_expectancy: 1.577},
    RE288 {balls: 1, strikes:  0, base_value: 3, outs: 0, run_expectancy: 1.567},
    RE288 {balls: 2, strikes:  1, base_value: 3, outs: 0, run_expectancy: 1.565},
    RE288 {balls: 3, strikes:  1, base_value: 4, outs: 0, run_expectancy: 1.549},
    RE288 {balls: 1, strikes:  1, base_value: 7, outs: 1, run_expectancy: 1.54},
    RE288 {balls: 2, strikes:  0, base_value: 6, outs: 1, run_expectancy: 1.514},
    RE288 {balls: 0, strikes:  0, base_value: 3, outs: 0, run_expectancy: 1.498},
    RE288 {balls: 2, strikes:  2, base_value: 7, outs: 1, run_expectancy: 1.497},
    RE288 {balls: 3, strikes:  1, base_value: 6, outs: 1, run_expectancy: 1.477},
    RE288 {balls: 2, strikes:  0, base_value: 4, outs: 0, run_expectancy: 1.476},
    RE288 {balls: 1, strikes:  1, base_value: 3, outs: 0, run_expectancy: 1.475},
    RE288 {balls: 3, strikes:  0, base_value: 5, outs: 1, run_expectancy: 1.46},
    RE288 {balls: 1, strikes:  0, base_value: 6, outs: 1, run_expectancy: 1.452},
    RE288 {balls: 0, strikes:  1, base_value: 7, outs: 1, run_expectancy: 1.452},
    RE288 {balls: 3, strikes:  0, base_value: 7, outs: 2, run_expectancy: 1.432},
    RE288 {balls: 1, strikes:  0, base_value: 4, outs: 0, run_expectancy: 1.429},
    RE288 {balls: 3, strikes:  2, base_value: 4, outs: 0, run_expectancy: 1.408},
    RE288 {balls: 0, strikes:  1, base_value: 3, outs: 0, run_expectancy: 1.407},
    RE288 {balls: 2, strikes:  2, base_value: 3, outs: 0, run_expectancy: 1.403},
    RE288 {balls: 2, strikes:  1, base_value: 6, outs: 1, run_expectancy: 1.392},
    RE288 {balls: 3, strikes:  2, base_value: 6, outs: 1, run_expectancy: 1.389},
    RE288 {balls: 3, strikes:  0, base_value: 2, outs: 0, run_expectancy: 1.383},
    RE288 {balls: 2, strikes:  1, base_value: 4, outs: 0, run_expectancy: 1.382},
    RE288 {balls: 0, strikes:  0, base_value: 4, outs: 0, run_expectancy: 1.378},
    RE288 {balls: 0, strikes:  0, base_value: 6, outs: 1, run_expectancy: 1.376},
    RE288 {balls: 2, strikes:  0, base_value: 5, outs: 1, run_expectancy: 1.359},
    RE288 {balls: 1, strikes:  1, base_value: 4, outs: 0, run_expectancy: 1.358},
    RE288 {balls: 1, strikes:  2, base_value: 7, outs: 1, run_expectancy: 1.354},
    RE288 {balls: 3, strikes:  0, base_value: 3, outs: 1, run_expectancy: 1.345},
    RE288 {balls: 1, strikes:  2, base_value: 3, outs: 0, run_expectancy: 1.343},
    RE288 {balls: 3, strikes:  1, base_value: 5, outs: 1, run_expectancy: 1.341},
    RE288 {balls: 0, strikes:  2, base_value: 7, outs: 1, run_expectancy: 1.334},
    RE288 {balls: 0, strikes:  1, base_value: 4, outs: 0, run_expectancy: 1.333},
    RE288 {balls: 1, strikes:  1, base_value: 6, outs: 1, run_expectancy: 1.329},
    RE288 {balls: 3, strikes:  1, base_value: 2, outs: 0, run_expectancy: 1.307},
    RE288 {balls: 2, strikes:  2, base_value: 4, outs: 0, run_expectancy: 1.305},
    RE288 {balls: 0, strikes:  2, base_value: 3, outs: 0, run_expectancy: 1.288},
    RE288 {balls: 2, strikes:  2, base_value: 6, outs: 1, run_expectancy: 1.283},
    RE288 {balls: 1, strikes:  0, base_value: 5, outs: 1, run_expectancy: 1.282},
    RE288 {balls: 1, strikes:  2, base_value: 4, outs: 0, run_expectancy: 1.278},
    RE288 {balls: 0, strikes:  2, base_value: 4, outs: 0, run_expectancy: 1.273},
    RE288 {balls: 0, strikes:  1, base_value: 6, outs: 1, run_expectancy: 1.267},
    RE288 {balls: 3, strikes:  0, base_value: 1, outs: 0, run_expectancy: 1.249},
    RE288 {balls: 2, strikes:  0, base_value: 2, outs: 0, run_expectancy: 1.249},
    RE288 {balls: 3, strikes:  1, base_value: 3, outs: 1, run_expectancy: 1.236},
    RE288 {balls: 2, strikes:  1, base_value: 5, outs: 1, run_expectancy: 1.234},
    RE288 {balls: 0, strikes:  0, base_value: 5, outs: 1, run_expectancy: 1.226},
    RE288 {balls: 3, strikes:  1, base_value: 7, outs: 2, run_expectancy: 1.217},
    RE288 {balls: 3, strikes:  2, base_value: 5, outs: 1, run_expectancy: 1.2},
    RE288 {balls: 1, strikes:  0, base_value: 2, outs: 0, run_expectancy: 1.2},
    RE288 {balls: 1, strikes:  2, base_value: 6, outs: 1, run_expectancy: 1.195},
    RE288 {balls: 2, strikes:  1, base_value: 2, outs: 0, run_expectancy: 1.194},
    RE288 {balls: 1, strikes:  1, base_value: 5, outs: 1, run_expectancy: 1.183},
    RE288 {balls: 3, strikes:  0, base_value: 4, outs: 1, run_expectancy: 1.164},
    RE288 {balls: 3, strikes:  2, base_value: 2, outs: 0, run_expectancy: 1.161},
    RE288 {balls: 2, strikes:  0, base_value: 7, outs: 2, run_expectancy: 1.157},
    RE288 {balls: 0, strikes:  0, base_value: 2, outs: 0, run_expectancy: 1.153},
    RE288 {balls: 3, strikes:  1, base_value: 1, outs: 0, run_expectancy: 1.146},
    RE288 {balls: 0, strikes:  1, base_value: 5, outs: 1, run_expectancy: 1.123},
    RE288 {balls: 0, strikes:  2, base_value: 6, outs: 1, run_expectancy: 1.118},
    RE288 {balls: 1, strikes:  1, base_value: 2, outs: 0, run_expectancy: 1.116},
    RE288 {balls: 3, strikes:  1, base_value: 4, outs: 1, run_expectancy: 1.108},
    RE288 {balls: 2, strikes:  0, base_value: 3, outs: 1, run_expectancy: 1.104},
    RE288 {balls: 0, strikes:  1, base_value: 2, outs: 0, run_expectancy: 1.104},
    RE288 {balls: 2, strikes:  2, base_value: 2, outs: 0, run_expectancy: 1.099},
    RE288 {balls: 3, strikes:  2, base_value: 3, outs: 1, run_expectancy: 1.086},
    RE288 {balls: 2, strikes:  2, base_value: 5, outs: 1, run_expectancy: 1.081},
    RE288 {balls: 2, strikes:  0, base_value: 1, outs: 0, run_expectancy: 1.078},
    RE288 {balls: 2, strikes:  0, base_value: 4, outs: 1, run_expectancy: 1.061},
    RE288 {balls: 1, strikes:  2, base_value: 5, outs: 1, run_expectancy: 1.051},
    RE288 {balls: 1, strikes:  2, base_value: 2, outs: 0, run_expectancy: 1.034},
    RE288 {balls: 1, strikes:  0, base_value: 4, outs: 1, run_expectancy: 1.023},
    RE288 {balls: 3, strikes:  2, base_value: 1, outs: 0, run_expectancy: 1.021},
    RE288 {balls: 2, strikes:  1, base_value: 4, outs: 1, run_expectancy: 1.021},
    RE288 {balls: 2, strikes:  1, base_value: 3, outs: 1, run_expectancy: 1.017},
    RE288 {balls: 1, strikes:  0, base_value: 3, outs: 1, run_expectancy: 1.011},
    RE288 {balls: 3, strikes:  2, base_value: 7, outs: 2, run_expectancy: 0.992},
    RE288 {balls: 0, strikes:  2, base_value: 2, outs: 0, run_expectancy: 0.989},
    RE288 {balls: 0, strikes:  2, base_value: 5, outs: 1, run_expectancy: 0.988},
    RE288 {balls: 0, strikes:  0, base_value: 4, outs: 1, run_expectancy: 0.987},
    RE288 {balls: 2, strikes:  1, base_value: 1, outs: 0, run_expectancy: 0.975},
    RE288 {balls: 1, strikes:  0, base_value: 1, outs: 0, run_expectancy: 0.969},
    RE288 {balls: 3, strikes:  2, base_value: 4, outs: 1, run_expectancy: 0.968},
    RE288 {balls: 1, strikes:  1, base_value: 4, outs: 1, run_expectancy: 0.958},
    RE288 {balls: 0, strikes:  0, base_value: 3, outs: 1, run_expectancy: 0.947},
    RE288 {balls: 1, strikes:  1, base_value: 3, outs: 1, run_expectancy: 0.932},
    RE288 {balls: 0, strikes:  1, base_value: 4, outs: 1, run_expectancy: 0.917},
    RE288 {balls: 1, strikes:  0, base_value: 7, outs: 2, run_expectancy: 0.916},
    RE288 {balls: 0, strikes:  0, base_value: 1, outs: 0, run_expectancy: 0.909},
    RE288 {balls: 1, strikes:  1, base_value: 1, outs: 0, run_expectancy: 0.896},
    RE288 {balls: 2, strikes:  1, base_value: 7, outs: 2, run_expectancy: 0.896},
    RE288 {balls: 2, strikes:  2, base_value: 3, outs: 1, run_expectancy: 0.891},
    RE288 {balls: 2, strikes:  2, base_value: 4, outs: 1, run_expectancy: 0.889},
    RE288 {balls: 0, strikes:  1, base_value: 3, outs: 1, run_expectancy: 0.884},
    RE288 {balls: 2, strikes:  2, base_value: 1, outs: 0, run_expectancy: 0.881},
    RE288 {balls: 3, strikes:  0, base_value: 2, outs: 1, run_expectancy: 0.87},
    RE288 {balls: 0, strikes:  1, base_value: 1, outs: 0, run_expectancy: 0.852},
    RE288 {balls: 1, strikes:  2, base_value: 4, outs: 1, run_expectancy: 0.837},
    RE288 {balls: 1, strikes:  2, base_value: 1, outs: 0, run_expectancy: 0.819},
    RE288 {balls: 1, strikes:  2, base_value: 3, outs: 1, run_expectancy: 0.81},
    RE288 {balls: 3, strikes:  0, base_value: 1, outs: 1, run_expectancy: 0.807},
    RE288 {balls: 3, strikes:  1, base_value: 2, outs: 1, run_expectancy: 0.802},
    RE288 {balls: 3, strikes:  0, base_value: 6, outs: 2, run_expectancy: 0.799},
    RE288 {balls: 0, strikes:  2, base_value: 4, outs: 1, run_expectancy: 0.796},
    RE288 {balls: 2, strikes:  0, base_value: 2, outs: 1, run_expectancy: 0.796},
    RE288 {balls: 0, strikes:  2, base_value: 1, outs: 0, run_expectancy: 0.788},
    RE288 {balls: 0, strikes:  2, base_value: 3, outs: 1, run_expectancy: 0.783},
    RE288 {balls: 0, strikes:  0, base_value: 7, outs: 2, run_expectancy: 0.77},
    RE288 {balls: 3, strikes:  0, base_value: 0, outs: 0, run_expectancy: 0.741},
    RE288 {balls: 1, strikes:  0, base_value: 2, outs: 1, run_expectancy: 0.739},
    RE288 {balls: 2, strikes:  1, base_value: 2, outs: 1, run_expectancy: 0.733},
    RE288 {balls: 3, strikes:  0, base_value: 3, outs: 2, run_expectancy: 0.729},
    RE288 {balls: 3, strikes:  2, base_value: 2, outs: 1, run_expectancy: 0.729},
    RE288 {balls: 3, strikes:  0, base_value: 5, outs: 2, run_expectancy: 0.728},
    RE288 {balls: 3, strikes:  1, base_value: 1, outs: 1, run_expectancy: 0.726},
    RE288 {balls: 3, strikes:  1, base_value: 6, outs: 2, run_expectancy: 0.725},
    RE288 {balls: 2, strikes:  0, base_value: 6, outs: 2, run_expectancy: 0.711},
    RE288 {balls: 1, strikes:  1, base_value: 7, outs: 2, run_expectancy: 0.707},
    RE288 {balls: 0, strikes:  0, base_value: 2, outs: 1, run_expectancy: 0.701},
    RE288 {balls: 2, strikes:  2, base_value: 7, outs: 2, run_expectancy: 0.698},
    RE288 {balls: 1, strikes:  1, base_value: 2, outs: 1, run_expectancy: 0.683},
    RE288 {balls: 3, strikes:  1, base_value: 3, outs: 2, run_expectancy: 0.668},
    RE288 {balls: 3, strikes:  1, base_value: 0, outs: 0, run_expectancy: 0.667},
    RE288 {balls: 3, strikes:  1, base_value: 5, outs: 2, run_expectancy: 0.666},
    RE288 {balls: 2, strikes:  0, base_value: 1, outs: 1, run_expectancy: 0.66},
    RE288 {balls: 1, strikes:  0, base_value: 6, outs: 2, run_expectancy: 0.651},
    RE288 {balls: 0, strikes:  1, base_value: 2, outs: 1, run_expectancy: 0.649},
    RE288 {balls: 2, strikes:  0, base_value: 5, outs: 2, run_expectancy: 0.645},
    RE288 {balls: 2, strikes:  2, base_value: 2, outs: 1, run_expectancy: 0.639},
    RE288 {balls: 0, strikes:  1, base_value: 7, outs: 2, run_expectancy: 0.638},
    RE288 {balls: 3, strikes:  2, base_value: 1, outs: 1, run_expectancy: 0.615},
    RE288 {balls: 2, strikes:  0, base_value: 0, outs: 0, run_expectancy: 0.612},
    RE288 {balls: 2, strikes:  1, base_value: 6, outs: 2, run_expectancy: 0.609},
    RE288 {balls: 2, strikes:  1, base_value: 1, outs: 1, run_expectancy: 0.592},
    RE288 {balls: 2, strikes:  0, base_value: 3, outs: 2, run_expectancy: 0.591},
    RE288 {balls: 0, strikes:  0, base_value: 6, outs: 2, run_expectancy: 0.587},
    RE288 {balls: 1, strikes:  2, base_value: 2, outs: 1, run_expectancy: 0.586},
    RE288 {balls: 3, strikes:  2, base_value: 0, outs: 0, run_expectancy: 0.584},
    RE288 {balls: 1, strikes:  0, base_value: 1, outs: 1, run_expectancy: 0.583},
    RE288 {balls: 0, strikes:  2, base_value: 2, outs: 1, run_expectancy: 0.58},
    RE288 {balls: 1, strikes:  1, base_value: 6, outs: 2, run_expectancy: 0.56},
    RE288 {balls: 2, strikes:  1, base_value: 0, outs: 0, run_expectancy: 0.553},
    RE288 {balls: 1, strikes:  0, base_value: 0, outs: 0, run_expectancy: 0.545},
    RE288 {balls: 0, strikes:  0, base_value: 1, outs: 1, run_expectancy: 0.543},
    RE288 {balls: 3, strikes:  2, base_value: 5, outs: 2, run_expectancy: 0.541},
    RE288 {balls: 1, strikes:  0, base_value: 5, outs: 2, run_expectancy: 0.54},
    RE288 {balls: 2, strikes:  1, base_value: 5, outs: 2, run_expectancy: 0.536},
    RE288 {balls: 3, strikes:  2, base_value: 6, outs: 2, run_expectancy: 0.525},
    RE288 {balls: 1, strikes:  1, base_value: 1, outs: 1, run_expectancy: 0.525},
    RE288 {balls: 2, strikes:  1, base_value: 3, outs: 2, run_expectancy: 0.524},
    RE288 {balls: 0, strikes:  2, base_value: 7, outs: 2, run_expectancy: 0.509},
    RE288 {balls: 0, strikes:  0, base_value: 0, outs: 0, run_expectancy: 0.508},
    RE288 {balls: 1, strikes:  0, base_value: 3, outs: 2, run_expectancy: 0.508},
    RE288 {balls: 1, strikes:  2, base_value: 7, outs: 2, run_expectancy: 0.507},
    RE288 {balls: 3, strikes:  0, base_value: 4, outs: 2, run_expectancy: 0.504},
    RE288 {balls: 0, strikes:  0, base_value: 5, outs: 2, run_expectancy: 0.501},
    RE288 {balls: 1, strikes:  1, base_value: 0, outs: 0, run_expectancy: 0.501},
    RE288 {balls: 0, strikes:  1, base_value: 6, outs: 2, run_expectancy: 0.499},
    RE288 {balls: 3, strikes:  2, base_value: 3, outs: 2, run_expectancy: 0.499},
    RE288 {balls: 0, strikes:  1, base_value: 1, outs: 1, run_expectancy: 0.495},
    RE288 {balls: 2, strikes:  2, base_value: 1, outs: 1, run_expectancy: 0.491},
    RE288 {balls: 2, strikes:  2, base_value: 0, outs: 0, run_expectancy: 0.488},
    RE288 {balls: 0, strikes:  1, base_value: 0, outs: 0, run_expectancy: 0.472},
    RE288 {balls: 3, strikes:  1, base_value: 4, outs: 2, run_expectancy: 0.463},
    RE288 {balls: 1, strikes:  1, base_value: 5, outs: 2, run_expectancy: 0.453},
    RE288 {balls: 0, strikes:  0, base_value: 3, outs: 2, run_expectancy: 0.453},
    RE288 {balls: 2, strikes:  2, base_value: 5, outs: 2, run_expectancy: 0.453},
    RE288 {balls: 1, strikes:  2, base_value: 1, outs: 1, run_expectancy: 0.449},
    RE288 {balls: 2, strikes:  0, base_value: 4, outs: 2, run_expectancy: 0.449},
    RE288 {balls: 1, strikes:  2, base_value: 0, outs: 0, run_expectancy: 0.444},
    RE288 {balls: 3, strikes:  0, base_value: 0, outs: 1, run_expectancy: 0.441},
    RE288 {balls: 1, strikes:  1, base_value: 3, outs: 2, run_expectancy: 0.435},
    RE288 {balls: 0, strikes:  1, base_value: 5, outs: 2, run_expectancy: 0.429},
    RE288 {balls: 0, strikes:  2, base_value: 1, outs: 1, run_expectancy: 0.429},
    RE288 {balls: 0, strikes:  2, base_value: 0, outs: 0, run_expectancy: 0.42},
    RE288 {balls: 3, strikes:  0, base_value: 2, outs: 2, run_expectancy: 0.419},
    RE288 {balls: 2, strikes:  2, base_value: 6, outs: 2, run_expectancy: 0.419},
    RE288 {balls: 2, strikes:  1, base_value: 4, outs: 2, run_expectancy: 0.399},
    RE288 {balls: 3, strikes:  2, base_value: 4, outs: 2, run_expectancy: 0.397},
    RE288 {balls: 3, strikes:  1, base_value: 0, outs: 1, run_expectancy: 0.395},
    RE288 {balls: 3, strikes:  1, base_value: 2, outs: 2, run_expectancy: 0.392},
    RE288 {balls: 1, strikes:  2, base_value: 6, outs: 2, run_expectancy: 0.391},
    RE288 {balls: 3, strikes:  0, base_value: 1, outs: 2, run_expectancy: 0.391},
    RE288 {balls: 0, strikes:  1, base_value: 3, outs: 2, run_expectancy: 0.389},
    RE288 {balls: 1, strikes:  0, base_value: 4, outs: 2, run_expectancy: 0.388},
    RE288 {balls: 2, strikes:  2, base_value: 3, outs: 2, run_expectancy: 0.387},
    RE288 {balls: 1, strikes:  2, base_value: 5, outs: 2, run_expectancy: 0.384},
    RE288 {balls: 2, strikes:  0, base_value: 2, outs: 2, run_expectancy: 0.379},
    RE288 {balls: 0, strikes:  0, base_value: 4, outs: 2, run_expectancy: 0.363},
    RE288 {balls: 0, strikes:  2, base_value: 6, outs: 2, run_expectancy: 0.359},
    RE288 {balls: 2, strikes:  0, base_value: 0, outs: 1, run_expectancy: 0.355},
    RE288 {balls: 1, strikes:  0, base_value: 2, outs: 2, run_expectancy: 0.353},
    RE288 {balls: 3, strikes:  1, base_value: 1, outs: 2, run_expectancy: 0.345},
    RE288 {balls: 1, strikes:  1, base_value: 4, outs: 2, run_expectancy: 0.344},
    RE288 {balls: 2, strikes:  1, base_value: 2, outs: 2, run_expectancy: 0.344},
    RE288 {balls: 1, strikes:  2, base_value: 3, outs: 2, run_expectancy: 0.331},
    RE288 {balls: 3, strikes:  2, base_value: 2, outs: 2, run_expectancy: 0.327},
    RE288 {balls: 3, strikes:  2, base_value: 0, outs: 1, run_expectancy: 0.325},
    RE288 {balls: 0, strikes:  0, base_value: 2, outs: 2, run_expectancy: 0.324},
    RE288 {balls: 0, strikes:  1, base_value: 4, outs: 2, run_expectancy: 0.323},
    RE288 {balls: 0, strikes:  2, base_value: 5, outs: 2, run_expectancy: 0.315},
    RE288 {balls: 2, strikes:  0, base_value: 1, outs: 2, run_expectancy: 0.312},
    RE288 {balls: 1, strikes:  1, base_value: 2, outs: 2, run_expectancy: 0.311},
    RE288 {balls: 2, strikes:  2, base_value: 4, outs: 2, run_expectancy: 0.31},
    RE288 {balls: 2, strikes:  1, base_value: 0, outs: 1, run_expectancy: 0.309},
    RE288 {balls: 1, strikes:  0, base_value: 0, outs: 1, run_expectancy: 0.304},
    RE288 {balls: 0, strikes:  1, base_value: 2, outs: 2, run_expectancy: 0.276},
    RE288 {balls: 0, strikes:  0, base_value: 0, outs: 1, run_expectancy: 0.276},
    RE288 {balls: 0, strikes:  2, base_value: 3, outs: 2, run_expectancy: 0.272},
    RE288 {balls: 1, strikes:  1, base_value: 0, outs: 1, run_expectancy: 0.269},
    RE288 {balls: 2, strikes:  2, base_value: 2, outs: 2, run_expectancy: 0.268},
    RE288 {balls: 1, strikes:  0, base_value: 1, outs: 2, run_expectancy: 0.266},
    RE288 {balls: 3, strikes:  2, base_value: 1, outs: 2, run_expectancy: 0.266},
    RE288 {balls: 2, strikes:  1, base_value: 1, outs: 2, run_expectancy: 0.262},
    RE288 {balls: 2, strikes:  2, base_value: 0, outs: 1, run_expectancy: 0.258},
    RE288 {balls: 1, strikes:  2, base_value: 4, outs: 2, run_expectancy: 0.249},
    RE288 {balls: 0, strikes:  1, base_value: 0, outs: 1, run_expectancy: 0.249},
    RE288 {balls: 0, strikes:  2, base_value: 4, outs: 2, run_expectancy: 0.238},
    RE288 {balls: 0, strikes:  0, base_value: 1, outs: 2, run_expectancy: 0.231},
    RE288 {balls: 1, strikes:  2, base_value: 2, outs: 2, run_expectancy: 0.228},
    RE288 {balls: 1, strikes:  2, base_value: 0, outs: 1, run_expectancy: 0.226},
    RE288 {balls: 1, strikes:  1, base_value: 1, outs: 2, run_expectancy: 0.221},
    RE288 {balls: 0, strikes:  2, base_value: 0, outs: 1, run_expectancy: 0.211},
    RE288 {balls: 0, strikes:  2, base_value: 2, outs: 2, run_expectancy: 0.203},
    RE288 {balls: 2, strikes:  2, base_value: 1, outs: 2, run_expectancy: 0.195},
    RE288 {balls: 0, strikes:  1, base_value: 1, outs: 2, run_expectancy: 0.189},
    RE288 {balls: 3, strikes:  0, base_value: 0, outs: 2, run_expectancy: 0.187},
    RE288 {balls: 1, strikes:  2, base_value: 1, outs: 2, run_expectancy: 0.166},
    RE288 {balls: 3, strikes:  1, base_value: 0, outs: 2, run_expectancy: 0.161},
    RE288 {balls: 2, strikes:  0, base_value: 0, outs: 2, run_expectancy: 0.145},
    RE288 {balls: 0, strikes:  2, base_value: 1, outs: 2, run_expectancy: 0.136},
    RE288 {balls: 3, strikes:  2, base_value: 0, outs: 2, run_expectancy: 0.127},
    RE288 {balls: 2, strikes:  1, base_value: 0, outs: 2, run_expectancy: 0.12},
    RE288 {balls: 1, strikes:  0, base_value: 0, outs: 2, run_expectancy: 0.119},
    RE288 {balls: 0, strikes:  0, base_value: 0, outs: 2, run_expectancy: 0.103},
    RE288 {balls: 1, strikes:  1, base_value: 0, outs: 2, run_expectancy: 0.099},
    RE288 {balls: 2, strikes:  2, base_value: 0, outs: 2, run_expectancy: 0.089},
    RE288 {balls: 0, strikes:  1, base_value: 0, outs: 2, run_expectancy: 0.086},
    RE288 {balls: 1, strikes:  2, base_value: 0, outs: 2, run_expectancy: 0.069},
    RE288 {balls: 0, strikes:  2, base_value: 0, outs: 2, run_expectancy: 0.06},
];