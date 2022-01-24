//! https://github.com/muzhig/ESOQ2/blob/master/esoq2p1.py
//! https://www.researchgate.net/publication/267422798_ESOQ-2_Single-Point_Algorithm_for_Fast_Optimal_Spacecraft_Attitude_Determination
pub fn esoq2 ( observations: List<Match<Cartesian3D> )
{

}

///
///
/// # Notes
/// K(4,4) = 
/// | B + B^T - I_3x3 * tr[B]  z      |
/// | Z^T                      tr[B]  |
///
/// K(4,4) = 
/// | S_00 - tr[B]		Z_1 + S_01,		Z_2 + S_02			Z_3 |
/// | Z_0 + S_10		tr[B]
/// | Z_1 + S_20		tr[B]
/// | Z_2 + S_30		tr[B]
///
pub fn find_k ( b: Matrix<3,3> )
{
	let b : Matrix<3,3> = find_b();
	let b_trace = b.trace();
	
	
	let z : Matrix<3, 1> = find_z();
}



/// Finds Z.
/// # Notes
/// The report is confusing as it specifies:
/// ```
/// Z = [b_23 - b_32,  b_31 - b_13,  b_12 - b_21]^T		(Equation 5) NOTE: lowest index is 1.
/// Z = [b_12 - b_21,  b_20 - b_02,  b_01 - b_10]^T		NOTE: Using lowest index 0.		
/// ```
/// However when it is used, it is referenced as transposed.  
/// When using z in the k matrix, it is a row (3, 1) matrix, do not consider the second transpose. 
///
/// # Returns
/// A row (3,1) matrix.
pub fn find_z ( b : Matrix<3,3> ) -> Matrix<3,1>
{
	let mut z : Matrix<3,1> = Matrix::new();
	z.set(MatPos{row: 0, col: 0}, b.get(MatPos{row: 1, col: 2} - b.get({MatPos{row: 2, col: 1}})));
	z.set(MatPos{row: 1, col: 0}, b.get(MatPos{row: 2, col: 0} - b.get({MatPos{row: 0, col: 2}})));
	z.set(MatPos{row: 2, col: 0}, b.get(MatPos{row: 0, col: 1} - b.get({MatPos{row: 1, col: 0}})));
	return z;
}




/// Finds the B matrix (3x3).  
/// # Notes
/// This is usualy done by providing 3 matrices:
/// ```
/// wt : (n, 1)		// A row matrix giving a relative weighting to each observation.
/// obs: (3, n)		// The values from the sensor (Cartesian).
/// act: (3, n)		// The actual locations (Cartesian).
/// '''
/// Consider each matrix being a row.  
/// 
/// Referencing the report, the equation would be:
/// ```
/// B = Sum(wt(i) * obs * ref')				(Equation 5)
/// ```
///
/// # Returns
/// A 3x3 matrix B.
pub fn find_b ( obs: List<Match<Cartesian3D>> ) -> Matrix<3, 3>
{
	let mut b : Matrix<3,3> = Matrix::new();
	for ii in 0..obs.size()
	{
		for jj in 0..obs.size()
		{
			let mut val;
			val += obs.get(ii).input.x * obs.get(jj).output.x; 
			val += obs.get(ii).input.y * obs.get(jj).output.y; 
			val += obs.get(ii).input.z * obs.get(jj).output.z; 
			val *= obs.weight;
			b.set(MatPos{row: ii, column: jj}, val);
		}
	}
}


///
pub fn find_lambda ( observations: List<Observation<Cartesian3D> )
{
	// Initial lambda guess is the sum of the weights. 
	lam = 0;
	for i in 0..lam.size()
	{
		lam += observations.get(i);
	}
	return 
}


// DAVENPORT
//
// σ = trace([B])
// Z = [B[2][3] - B[3][2],		B[3][1] - B[1][3],		B[1][2] - B[2][1]] ^ T
// S = [B] + [B] ^ T
// [B] = \sum(w_i W_i A V^2_i)
//
// λ = Eigenvector([K]) 		<- OUTPUT
//
//		| σ			Z[1]		Z[2]		Z[3]		|
// K = 	| Z[1]		S[1][1] -σ	S[1][2]		S[1][3]		|
// 		| Z[2]		S[2][1]		S[2][2] -σ	S[2][3]		|
// 		| Z[3]		S[3][1]		S[3][2]		S[3][3] -σ	|


def esoq2p1(obs, ref, wt):
    lam = sum(wt)  												# zeroth order approximation to lambda_max
    B = np.array([obs[0, :]*wt, obs[1, :]*wt, obs[2, :]*wt]) 	# converts to array * weight ?
    B = B.dot(ref.T) 											# ??? 
    trB = np.trace(B)											# **SIGMA**
    diag = [B[0, 0], B[1, 1], B[2, 2], trB]  					# ?


	# # #
    # Optimal 180 deg rotation to avoid zero rotation angle singularity
	# # #

    Bmin = min(diag)
    irot = diag.index(Bmin)

    if irot == 0:
        B[:, 1:3] *= -1
        trB = 2 * Bmin - trB
    elif irot == 1:
        B[:, 0] *= -1
        B[:, 2] *= -1
        trB = 2 * Bmin - trB
    elif irot == 2:
        B[:, 0:2] *= -1
        trB = 2 * Bmin - trB


	# # #
    # Compute needed matrices and vectors
    # # #
	S11 = 2 * B[0, 0]			# **S**
    S23 = B[1, 2] + B[2, 1]		# The diagonal is identical transposed.
    S22 = 2 * B[1, 1]			#
    S31 = B[2, 0] + B[0, 2]		#
    S33 = 2 * B[2, 2]			#
    S12 = B[0, 1] + B[1, 0]		#
	
    z = np.array([B[1, 2] - B[2, 1], B[2, 0] - B[0, 2], B[0, 1] - B[1, 0]]) # Finds Z (uses 0 as 1)
    z12 = z[0] * z[0]
    z22 = z[1] * z[1]	# ???
    z32 = z[2] * z[2]

	# IF ONLY 2 VALUES
    wt_len_eq_2 = max(wt.shape) == 2
    # max eigenvalue computation for two observation case
    if wt_len_eq_2:
        lam0 = lam
        trB2 = trB * trB
        Sz = np.array([[S11, S12, S31], [S12, S22, S23], [S31, S23, S33]]).dot(z)
        aa = trB2 - S22 * S33 + S23 * S23 - S11 * S33 + S31 * S31 - S22 * S11 + S12 * S12
        bb = trB2 + z12 + z22 + z32
        c2 = - aa - bb
        u = 2 * np.sqrt(aa * bb - Sz.T.dot(Sz))

        lam = (np.sqrt(u - c2) + np.sqrt(- u - c2)) / 2
        loss = lam0 - lam
		
		
		
    tml = trB - lam
    tpl = trB + lam


    M11 = tml * (S11 - tpl) - z12
    M23 = tml * S23 - z[1] * z[2]
    M22 = tml * (S22 - tpl) - z22
    M31 = tml * S31 - z[2] * z[0]
    M33 = tml * (S33 - tpl) - z32
    M12 = tml * S12 - z[0] * z[1]





	# # #
    # Compute loss function and rotation axis
    # # #
	
	e = np.array([M22 * M33 - M23 * M23, M11 * M33 - M31 * M31, M11 * M22 - M12 * M12])

    dummy = np.max(np.abs(e))

    if e[0] == dummy:
        e = np.array([e[0], M31 * M23 - M12 * M33, M12 * M23 - M31 * M22])
        imax = 0
    elif e[1] == dummy:
        e = np.array([M31 * M23 - M12 * M33, e[1], M12 * M31 - M11 * M23])
        imax = 1
    else:
        e = np.array([M12 * M23 - M31 * M22, M12 * M31 - M11 * M23, e[2]])
        imax = 2

    if not wt_len_eq_2:
        m1 = np.array([M11, M12, M31])
        m2 = np.array([M12, M22, M23])
        m3 = np.array([M31, M23, M33])
        n1 = np.array([(S11 - 2 * lam), S12, S31])
        n2 = np.array([S12, (S22 - 2 * lam), S23])
        n3 = np.array([S31, S23, (S33 - 2 * lam)])


        a = [m2, m3, m1][imax]
        b = [n3, n1, n2][imax]
        c = [m3, m1, m2][imax]
        d = [n2, n3, n1][imax]
        m = [m1, m2, m3][imax]
        n = [n1, n2, n3][imax]

        v = np.cross(a,b).T - np.cross(c, d).T

        loss = - (m.dot(e)) / (n.dot(e) + m.dot(v))
        tml = tml + loss
        e = e + loss * v
    # Quaternion computation in rotated frame
    q = np.hstack((tml * e, -z.T.dot(e)))

    q = q / np.linalg.norm(q)
    # Undo rotation to get quaternion in input frame
    if irot == 0:
        q = np.array([-q[0], q[3], -q[2], q[1]])
    elif irot == 1:
        q = np.array([-q[1], q[2], q[3], -q[0]])
    elif irot == 2:
        q = np.array([-q[2], -q[1], q[0], q[3]])

    return q, loss