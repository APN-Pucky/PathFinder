from math import isclose
import numpy as np
import pathfinder as pf
import rustypathfinder as rpf

N =  200                                                                         # Matrix size
p = 0.5   

seed = 0
np.random.seed(seed) 
print(f"seed = {seed}")
                                                                  # Distribution of True values 
pseudo = np.triu(np.random.choice([True, False], size=(N,N), p=[p, 1-p]), 1)    # Construct BAM values 
pseudo += pseudo.T          
np.savetxt("pseudo.csv", pseudo, delimiter=",")

seed = 0
np.random.seed(seed) 
print(f"seed = {seed}")
weights= np.sort(np.random.rand(N))[::-1]                                       # Generate pseudo weights (descending order)
np.savetxt("weights.csv", weights, delimiter=",")

pseudo = np.loadtxt("pseudo.csv", delimiter=",").astype(int) > 0.5
weights = np.loadtxt("weights.csv", delimiter=",")

def path_numpy_slow(matrix=pseudo):
    #assert matrix.shape[0] == matrix.shape[1]
    # print(matrix.shape)
    rets = []
    for i in range(len(matrix)):
        rets += [np.array([i])]
        for j in range(i,len(matrix[0])):
            if matrix[i,j]:
                #ret = [np.array([i,j])]
                ret = []
                mask = matrix[i] & (np.concatenate((np.zeros(j), np.ones(len(matrix[i])-j)) ).astype(bool))
                steps= np.array([i for i in range(len(mask)) if mask[i]])
                #print(mask)
                for p in path_numpy(matrix[:, mask][mask , :]):
                    # add to p the number of steps made in mask
                    ret += [np.array([i, *(steps[p])])]
                rets += ret
    return rets

def _path_numpy(matrix=pseudo):
    #assert matrix.shape[0] == matrix.shape[1]
    # print(matrix.shape)
    rets = []
    for j in range(len(matrix[0])):
        if matrix[0,j]:
            #ret = [np.array([i,j])]
            ret = [np.array([j])]
            mask = matrix[0] & (np.concatenate((np.zeros(j), np.ones(len(matrix[0])-j)) ).astype(bool))
            steps= np.array([i for i in range(len(mask)) if mask[i]])
            #print(mask)
            for p in _path_numpy(matrix[:, mask][mask , :]):
                # add to p the number of steps made in mask
                ret += [np.array([j, *(steps[p])])]
            rets += ret
    return rets

def path_numpy(matrix=pseudo):
    rets = []
    for i in range(len(matrix)):
        rets += [np.array( [i,*(path+i)]) for path in _path_numpy(matrix[i:,i:])]
    return rets

def get_weighted_path_numpy(matrix=pseudo, weights=weights):
    paths = path_numpy(matrix)
    weightsum = np.array([sum([weights[i] for i in path]) for path in paths])
    mask = np.argsort(weightsum)
    # return sorted paths
    return [np.unique(paths[mask[i]]) for i in range(len(mask))],weightsum[mask]


def get_weighted_path_rust(matrix=pseudo, weights=weights):
    paths = rpf.rec()
    weightsum = np.array([sum([weights[i] for i in path]) for path in paths])
    mask = np.argsort(weightsum)
    # return sorted paths
    return [np.unique(paths[mask[i]]) for i in range(len(mask))],weightsum[mask]



def get_pseudo():

    return pseudo

def get_weights():

    return weights

def get_bam(pseudo=None, weights=None):
    if pseudo is None:
        pseudo = get_pseudo()
    if weights is None:
        weights = get_weights()
    bam = pf.BinaryAcceptance(pseudo, weights=weights)      
    return bam

def hdfs():
    bam = get_bam()
    hdfs = pf.HDFS(bam, top=5, ignore_subset=True)
    hdfs.find_paths(verbose=True)

def whdfs():
    bam = get_bam()
    hdfs = pf.HDFS(bam, top=5, ignore_subset=True)
    hdfs.find_paths(verbose=True)

def test_hdfs(benchmark):
    benchmark(hdfs)

def test_whdfs(benchmark):
    benchmark(whdfs)

#def test_path_numpy(benchmark):
#    benchmark(path_numpy, pseudo)

def test_rusty(benchmark):
    benchmark(rpf.rec)
